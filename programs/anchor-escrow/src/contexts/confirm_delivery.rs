use anchor_lang::prelude::*;
use anchor_spl::token::{TransferChecked, transfer_checked, Token, TokenAccount, Mint};

use crate::states::Escrow;
use crate::ErrorCode;

#[derive(Accounts)]
pub struct ConfirmDelivery<'info> {
    /// CHECK: Used for admin verification
    #[account(mut, signer)]
    pub luxhub: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"state", escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = escrow
    )]
    pub nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub wsol_vault: Box<Account<'info, TokenAccount>>,

    pub mint_a: Box<Account<'info, Mint>>, // wSOL
    pub mint_b: Box<Account<'info, Mint>>, // NFT mint

    #[account(mut)]
    pub seller_funds_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub luxhub_fee_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub seller_nft_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub buyer_nft_ata: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

pub fn confirm_delivery(ctx: Context<ConfirmDelivery>) -> Result<()> {
    let bump = ctx.accounts.escrow.bump;
    let seed = ctx.accounts.escrow.seed;
    let initializer_amount = ctx.accounts.escrow.initializer_amount;
    let sale_price = ctx.accounts.escrow.sale_price;

    let escrow = &mut ctx.accounts.escrow;

    require_keys_eq!(
        ctx.accounts.buyer_nft_ata.owner,
        escrow.buyer,
        ErrorCode::Unauthorized
    );

    let signer_seeds: &[&[u8]] = &[b"state", &seed.to_le_bytes(), &[bump]];

    // Transfer NFT to buyer
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.nft_vault.to_account_info(),
                mint: ctx.accounts.mint_b.to_account_info(),
                to: ctx.accounts.buyer_nft_ata.to_account_info(),
                authority: escrow.to_account_info(),
            },
            &[signer_seeds],
        ),
        initializer_amount,
        ctx.accounts.mint_b.decimals,
    )?;

    // Calculate splits
    let seller_share = sale_price
        .checked_mul(95)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(100)
        .ok_or(ErrorCode::MathOverflow)?;

    let fee_share = sale_price.checked_sub(seller_share).ok_or(ErrorCode::MathOverflow)?;

    // Transfer 5% to LuxHub
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.wsol_vault.to_account_info(),
                mint: ctx.accounts.mint_a.to_account_info(),
                to: ctx.accounts.luxhub_fee_ata.to_account_info(),
                authority: escrow.to_account_info(),
            },
            &[signer_seeds],
        ),
        fee_share,
        ctx.accounts.mint_a.decimals,
    )?;

    // Transfer 95% to Seller
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.wsol_vault.to_account_info(),
                mint: ctx.accounts.mint_a.to_account_info(),
                to: ctx.accounts.seller_funds_ata.to_account_info(),
                authority: escrow.to_account_info(),
            },
            &[signer_seeds],
        ),
        seller_share,
        ctx.accounts.mint_a.decimals,
    )?;

    // Finalize
    escrow.is_completed = true;

    msg!(
        "✅ Delivery confirmed: NFT transferred, {} to seller, {} to treasury",
        seller_share,
        fee_share
    );

    Ok(())
}
