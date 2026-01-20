use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub mod constants;
pub mod errors;
pub mod utils;
pub mod state;
pub mod instructions;

use state::{Escrow, EscrowConfig};
use constants::{CONFIG_SEED, ESCROW_SEED};
use errors::LuxError;

declare_id!("kW2w2pHhAP8hFGRLganziunchKu6tjaXyomvF6jxNpj");

// ============================================
// ACCOUNT CONTEXTS (inline for Anchor 0.31.0 compatibility)
// ============================================

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + EscrowConfig::SIZE,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, EscrowConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, EscrowConfig>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        constraint = seller_ata_a.mint == mint_a.key(),
        constraint = seller_ata_a.owner == seller.key()
    )]
    pub seller_ata_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = seller_ata_b.mint == mint_b.key(),
        constraint = seller_ata_b.owner == seller.key(),
        constraint = seller_ata_b.amount == 1
    )]
    pub seller_ata_b: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        space = 8 + Escrow::SIZE,
        seeds = [ESCROW_SEED, &seed.to_le_bytes()[..]],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = admin,
        token::mint = mint_b,
        token::authority = escrow
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        token::mint = mint_a,
        token::authority = escrow
    )]
    pub wsol_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub escrow: Account<'info, Escrow>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(mut)]
    pub taker_funds_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub wsol_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDelivery<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,

    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, EscrowConfig>,

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

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AdminOnlyExample<'info> {
    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, EscrowConfig>,

    /// CHECK: Verified in handler against `config.squads_authority`
    pub authority: AccountInfo<'info>,

    /// CHECK: Read-only sysvar
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, &escrow.seed.to_le_bytes()[..]],
        bump = escrow.bump,
        constraint = escrow.initializer == seller.key() @ LuxError::NotSeller,
        constraint = !escrow.is_completed @ LuxError::EscrowAlreadyCompleted,
        constraint = escrow.buyer == Pubkey::default() @ LuxError::EscrowHasBuyer,
    )]
    pub escrow: Account<'info, Escrow>,
}

// ============================================
// PROGRAM INSTRUCTIONS
// ============================================

#[program]
pub mod luxhub_marketplace {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        squads_multisig: Pubkey,
        squads_authority: Pubkey,
    ) -> Result<()> {
        instructions::initialize_config::handler(ctx, squads_multisig, squads_authority)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
        file_cid: String,
        sale_price: u64,
    ) -> Result<()> {
        let bump: u8 = ctx.bumps.escrow;
        instructions::initialize::handler(
            ctx, seed, bump, initializer_amount, taker_amount, file_cid, sale_price,
        )
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        instructions::exchange::handler(ctx)
    }

    pub fn confirm_delivery(ctx: Context<ConfirmDelivery>) -> Result<()> {
        instructions::confirm_delivery::handler(ctx)
    }

    pub fn admin_only_example(ctx: Context<AdminOnlyExample>) -> Result<()> {
        instructions::admin_only_example::handler(ctx)
    }

    /// Update the sale price of an escrow listing.
    /// Only callable by the original seller before a buyer is assigned.
    pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
        instructions::update_price::handler(ctx, new_price)
    }
}
