use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        self, Transfer, Token, TokenAccount, Mint
    },
};

use crate::states::Escrow;

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub taker: Signer<'info>, // Buyer

    #[account(
        mut,
        seeds = [b"state", escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub mint_a: Account<'info, Mint>, // Funds mint (wSOL)
    pub mint_b: Account<'info, Mint>, // NFT mint

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker
    )]
    pub taker_funds_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = taker
    )]
    pub taker_nft_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Account<'info, TokenAccount>, // 🏦 Vault for wSOL

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_exchange(ctx: Context<Exchange>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    msg!("🔐 Locking in buyer to escrow...");
    escrow.buyer = ctx.accounts.taker.key();

    let amount = escrow.sale_price;
    msg!("💸 Transferring {} lamports of wSOL from buyer to vault", amount);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.taker_funds_ata.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.taker.to_account_info(),
            },
        ),
        amount,
    )?;

    msg!("✅ Buyer locked and funds deposited");
    Ok(())
}
