use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{Escrow, EscrowConfig};
use crate::constants::CONFIG_SEED;
use anchor_lang::{prelude::*, solana_program::sysvar::instructions as ix_sysvar};
use crate::constants::SQUADS_V4_PUBKEY;
use crate::errors::LuxError;

#[derive(Accounts)]
pub struct ConfirmDelivery<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,

    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, EscrowConfig>,

    // All of these will be validated in the handler
    #[account(mut)]
    pub buyer_nft_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub nft_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub wsol_vault: Account<'info, TokenAccount>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(mut)]
    pub seller_funds_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub luxhub_fee_ata: Account<'info, TokenAccount>,

    /// CHECK: verified in handler
    pub authority: AccountInfo<'info>,

    /// CHECK: we only read its bytes
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
}
