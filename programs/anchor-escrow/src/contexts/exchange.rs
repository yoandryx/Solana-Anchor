use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        self, close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, TransferChecked, Transfer,
    },
};

use crate::states::Escrow;
use crate::states::admin_list::AdminList;

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub initializer: SystemAccount<'info>,
    pub mint_a: Box<Account<'info, Mint>>,
    pub mint_b: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker
    )]
    pub taker_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker
    )]
    pub taker_ata_b: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = initializer
    )]
    pub initializer_ata_b: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        has_one = mint_b,
        constraint = taker_ata_b.amount >= escrow.taker_amount,
        close = initializer,
        seeds=[b"state", escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Exchange<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        transfer_checked(
            self.into_deposit_context(),
            self.escrow.taker_amount,
            self.mint_b.decimals,
        )
    }

    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"state",
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        transfer_checked(
            self.into_withdraw_context().with_signer(&signer_seeds),
            self.escrow.initializer_amount,
            self.mint_a.decimals,
        )?;

        close_account(self.into_close_context().with_signer(&signer_seeds))
    }

    fn into_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.initializer_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn into_withdraw_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.initializer.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

/// New instruction: ConfirmDelivery.
/// This instruction is to be invoked by a LuxHub admin (whose key must match escrow.luxhub_wallet)
/// to confirm that the buyer received the watch. Upon confirmation, funds are distributed (97% to the seller,
/// 3% fee to LuxHub) and the NFT is transferred from the seller to the buyer.
#[derive(Accounts)]
pub struct ConfirmDelivery<'info> {
    /// The LuxHub admin confirming delivery.
    #[account(mut, signer)]
    /// CHECK: This account is only used for signing and verifying that the admin's key matches the escrow's luxhub_wallet. No data is read from this account.
    pub luxhub: AccountInfo<'info>,
    /// The escrow account.
    #[account(mut, seeds = [b"state", escrow.seed.to_le_bytes().as_ref()], bump = escrow.bump)]
    pub escrow: Account<'info, Escrow>,
    /// The vault account holding funds.
    #[account(mut, associated_token::mint = mint_a, associated_token::authority = escrow)]
    pub vault: Account<'info, TokenAccount>,
    /// The funds mint account (mint_a).
    pub mint_a: Account<'info, Mint>,
    /// Seller's funds receiving token account.
    #[account(mut)]
    pub seller_funds_ata: Account<'info, TokenAccount>,
    /// LuxHub's fee receiving token account.
    #[account(mut)]
    pub luxhub_fee_ata: Account<'info, TokenAccount>,
    /// The NFT mint (assumed to be the same as mint_a for funds; adjust if different).
    pub nft_mint: Account<'info, Mint>,
    /// Seller's NFT token account (holding the NFT).
    #[account(mut)]
    pub seller_nft_ata: Account<'info, TokenAccount>,
    /// Buyer's NFT token account (to receive the NFT).
    #[account(mut)]
    pub buyer_nft_ata: Account<'info, TokenAccount>,
    /// The AdminList account for authorization.
    pub admin_list: Account<'info, AdminList>,
    pub token_program: Program<'info, Token>,
}

pub fn confirm_delivery(ctx: Context<ConfirmDelivery>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;
    // Ensure the LuxHub signer matches the stored luxhub_wallet.
    require!(*ctx.accounts.luxhub.key == escrow.luxhub_wallet, CustomError::Unauthorized);

    // Calculate fee: 3% of initializer_amount.
    let total = escrow.initializer_amount;
    let fee = total * 3 / 100;
    let seller_amount = total
        .checked_sub(fee)
        .ok_or(ErrorCode::CalculationError)?;

    // Transfer seller_amount from vault to seller.
    {
        // Bind the seed bytes so they live long enough.
        let seed_bytes: [u8; 8] = escrow.seed.to_le_bytes();
        let signer_seeds: &[&[u8]] = &[b"state", &seed_bytes, &[escrow.bump]];
        let cpi_accounts: TransferChecked<'_> = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(), // using mint_a's decimals for funds
            to: ctx.accounts.seller_funds_ata.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };
        let signer_seeds_slice: &[&[&[u8]]; 1] = &[signer_seeds];
        let cpi_ctx: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds_slice,
        );
        transfer_checked(cpi_ctx, seller_amount, ctx.accounts.mint_a.decimals)?;
    }

    // Transfer fee from vault to LuxHub fee account.
    {
        let seed_bytes: [u8; 8] = escrow.seed.to_le_bytes();
        let signer_seeds: &[&[u8]] = &[b"state", &seed_bytes, &[escrow.bump]];
        let cpi_accounts: TransferChecked<'_> = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.luxhub_fee_ata.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };
        let signer_seeds_slice: &[&[&[u8]]; 1] = &[signer_seeds];
        let cpi_ctx: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds_slice,
        );
        transfer_checked(cpi_ctx, fee, ctx.accounts.mint_a.decimals)?;
    }

    // Transfer NFT from seller to buyer.
    {
        // Using a standard token transfer (amount = 1 for NFT).
        let cpi_accounts: Transfer<'_> = Transfer {
            from: ctx.accounts.seller_nft_ata.to_account_info(),
            to: ctx.accounts.buyer_nft_ata.to_account_info(),
            authority: ctx.accounts.luxhub.to_account_info(), // LuxHub acts as admin here
        };
        let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;
    }

    // Optionally, you could close the vault account here if it’s empty.

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Calculation error.")]
    CalculationError,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access.")]
    Unauthorized,
}
