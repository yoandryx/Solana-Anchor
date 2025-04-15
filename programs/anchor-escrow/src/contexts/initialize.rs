use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        self, 
        Transfer, 
        Token, 
        TokenAccount, 
        Mint,
    },
};
use crate::states::Escrow;

#[derive(Accounts)]
#[instruction(
    seed: u64,
    initializer_amount: u64,
    taker_amount: u64,
    file_cid: String,
    luxhub_wallet: Pubkey,
    sale_price: u64,
    buyer: Pubkey,
)]

pub struct Initialize<'info> {
    /// The admin is the signer paying for account creation.
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The seller who holds the NFT and approves the listing.
    /// Must be a signer so we can transfer the NFT.
    #[account(mut)]
    pub seller: Signer<'info>,

    /// The funds mint (usually wrapped SOL).
    pub mint_a: Account<'info, Mint>,

    /// The NFT mint (the watch NFT).
    pub mint_b: Account<'info, Mint>,

    /// Seller's ATA for mint_a (funds).
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = seller
    )]
    pub seller_ata_a: Account<'info, TokenAccount>,

    /// Seller's ATA for the NFT mint (mint_b).
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = seller
    )]
    pub seller_ata_b: Account<'info, TokenAccount>,

    /// The escrow account (holds the essential info for the trade).
    #[account(
        init,
        payer = admin,
        space = Escrow::INIT_SPACE,
        seeds = [b"state", seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// The vault ATA for the NFT, owned by the escrow.
    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_b,
        associated_token::authority = escrow
    )]
    pub vault: Account<'info, TokenAccount>,

    /// Programs + system accounts
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Calculation error.")]
    CalculationError,
    #[msg("Vault account does not have enough tokens.")]
    VaultInsufficientFunds,
    #[msg("Insufficient NFT balance in seller account.")]
    InsufficientNFTBalance,
}

impl<'info> Initialize<'info> {
    pub fn initialize_escrow(
        &mut self,
        seed: u64,
        escrow_bump: u8,
        initializer_amount: u64,
        taker_amount: u64,
        file_cid: String,
        luxhub_wallet: Pubkey,
        sale_price: u64,
        buyer: Pubkey, // ✅ NEW
    ) -> Result<()> {
        // Populate escrow data.
        self.escrow.seed = seed;
        self.escrow.bump = escrow_bump;
        self.escrow.initializer = self.seller.key();
        self.escrow.luxhub_wallet = luxhub_wallet;
        self.escrow.mint_a = self.mint_a.key();
        self.escrow.mint_b = self.mint_b.key();
        self.escrow.initializer_amount = initializer_amount;
        self.escrow.taker_amount = taker_amount;
        self.escrow.file_cid = file_cid;
        self.escrow.sale_price = sale_price;
        self.escrow.buyer = buyer; // ✅ NEW

        // Log the current NFT balance from seller_ata_b.
        msg!("Seller ATA (NFT) balance: {}", self.seller_ata_b.amount);

        // Ensure seller_ata_b holds at least the expected number of tokens.
        require!(
            self.seller_ata_b.amount >= initializer_amount,
            ErrorCode::InsufficientNFTBalance
        );

        // Automatically deposit the NFT from seller_ata_b into the vault.
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.seller_ata_b.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.seller.to_account_info(),
                },
            ),
            initializer_amount, // Typically 1 for an NFT.
        )?;

        Ok(())
    }
}
