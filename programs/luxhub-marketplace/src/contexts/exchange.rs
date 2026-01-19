use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub escrow: Account<'info, Escrow>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    // Buyer funds ATA (we’ll check in handler)
    #[account(mut)]
    pub taker_funds_ata: Account<'info, TokenAccount>,

    // Escrow funds vault (created in Initialize)
    #[account(mut)]
    pub wsol_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
