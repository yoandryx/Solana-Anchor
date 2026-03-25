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

    #[account(
        mut,
        seeds = [ESCROW_SEED, &escrow.seed.to_le_bytes()[..]],
        bump = escrow.bump,
        constraint = !escrow.is_completed @ LuxError::EscrowAlreadyCompleted,
        constraint = escrow.buyer == Pubkey::default() @ LuxError::EscrowHasBuyer,
        constraint = escrow.mint_a == mint_a.key() @ LuxError::MintMismatch,
        constraint = escrow.mint_b == mint_b.key() @ LuxError::MintMismatch,
    )]
    pub escrow: Account<'info, Escrow>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        constraint = taker_funds_ata.mint == mint_a.key() @ LuxError::MintMismatch,
        constraint = taker_funds_ata.owner == taker.key() @ LuxError::Unauthorized,
    )]
    pub taker_funds_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = wsol_vault.owner == escrow.key() @ LuxError::Unauthorized,
        constraint = wsol_vault.mint == mint_a.key() @ LuxError::MintMismatch,
    )]
    pub wsol_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmDelivery<'info> {
    #[account(
        mut,
        seeds = [ESCROW_SEED, &escrow.seed.to_le_bytes()[..]],
        bump = escrow.bump,
        constraint = !escrow.is_completed @ LuxError::EscrowAlreadyCompleted,
        constraint = escrow.buyer != Pubkey::default() @ LuxError::Unauthorized,
        constraint = escrow.mint_a == mint_a.key() @ LuxError::MintMismatch,
        constraint = escrow.mint_b == mint_b.key() @ LuxError::MintMismatch,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, EscrowConfig>,

    #[account(
        mut,
        constraint = buyer_nft_ata.owner == escrow.buyer @ LuxError::Unauthorized,
        constraint = buyer_nft_ata.mint == mint_b.key() @ LuxError::MintMismatch,
    )]
    pub buyer_nft_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = nft_vault.owner == escrow.key() @ LuxError::Unauthorized,
        constraint = nft_vault.mint == mint_b.key() @ LuxError::MintMismatch,
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = wsol_vault.owner == escrow.key() @ LuxError::Unauthorized,
        constraint = wsol_vault.mint == mint_a.key() @ LuxError::MintMismatch,
    )]
    pub wsol_vault: Account<'info, TokenAccount>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        constraint = seller_funds_ata.owner == escrow.initializer @ LuxError::Unauthorized,
        constraint = seller_funds_ata.mint == mint_a.key() @ LuxError::MintMismatch,
    )]
    pub seller_funds_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = luxhub_fee_ata.owner == config.treasury @ LuxError::Unauthorized,
        constraint = luxhub_fee_ata.mint == mint_a.key() @ LuxError::MintMismatch,
    )]
    pub luxhub_fee_ata: Account<'info, TokenAccount>,

    /// CHECK: verified in handler against config.authority
    pub authority: AccountInfo<'info>,

    /// CHECK: we only read its bytes
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
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

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, &escrow.seed.to_le_bytes()[..]],
        bump = escrow.bump,
        constraint = escrow.initializer == seller.key() @ LuxError::NotSeller,
        constraint = !escrow.is_completed @ LuxError::EscrowAlreadyCompleted,
        constraint = escrow.buyer == Pubkey::default() @ LuxError::CannotCancelWithBuyer,
        close = seller
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        constraint = nft_vault.owner == escrow.key(),
        constraint = nft_vault.mint == escrow.mint_b
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = wsol_vault.owner == escrow.key(),
        constraint = wsol_vault.mint == escrow.mint_a
    )]
    pub wsol_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = seller_nft_ata.owner == seller.key(),
        constraint = seller_nft_ata.mint == escrow.mint_b
    )]
    pub seller_nft_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RefundBuyer<'info> {
    #[account(
        mut,
        seeds = [ESCROW_SEED, &escrow.seed.to_le_bytes()[..]],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, EscrowConfig>,

    #[account(
        mut,
        constraint = buyer_funds_ata.owner == escrow.buyer,
        constraint = buyer_funds_ata.mint == escrow.mint_a
    )]
    pub buyer_funds_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = funds_vault.owner == escrow.key(),
        constraint = funds_vault.mint == escrow.mint_a
    )]
    pub funds_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = nft_vault.owner == escrow.key(),
        constraint = nft_vault.mint == escrow.mint_b
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = seller_nft_ata.owner == escrow.initializer,
        constraint = seller_nft_ata.mint == escrow.mint_b
    )]
    pub seller_nft_ata: Account<'info, TokenAccount>,

    /// CHECK: verified in handler
    pub authority: AccountInfo<'info>,

    /// CHECK: we only read its bytes
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        constraint = admin.key() == config.authority @ LuxError::Unauthorized,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, EscrowConfig>,
}

#[derive(Accounts)]
pub struct CloseConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: We intentionally use AccountInfo to close accounts with mismatched data layouts.
    /// The PDA seeds verification ensures this is the correct config account.
    /// Authority check is performed in the handler by reading the first 40 bytes
    /// (8-byte discriminator + 32-byte authority pubkey).
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: AccountInfo<'info>,
}

// ============================================
// PROGRAM INSTRUCTIONS
// ============================================

#[program]
pub mod luxhub_marketplace {
    use super::*;

    /// Initialize the protocol config. Can only be called once.
    /// authority: The multisig that controls config updates
    /// treasury: The vault where fees are sent
    /// fee_bps: Fee in basis points (300 = 3%)
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        authority: Pubkey,
        treasury: Pubkey,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize_config::handler(ctx, authority, treasury, fee_bps)
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

    /// Update the sale price of an escrow listing.
    /// Only callable by the original seller before a buyer is assigned.
    pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
        instructions::update_price::handler(ctx, new_price)
    }

    /// Cancel an escrow listing and return the NFT to seller.
    /// Only callable by the original seller before a buyer deposits funds.
    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        instructions::cancel_escrow::handler(ctx)
    }

    /// Refund buyer's USDC and return NFT to seller.
    /// Only callable via Squads multisig CPI.
    pub fn refund_buyer(ctx: Context<RefundBuyer>) -> Result<()> {
        instructions::refund_buyer::handler(ctx)
    }

    /// Update the protocol config. All parameters are optional.
    /// Only callable by the current authority (should be multisig).
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_authority: Option<Pubkey>,
        new_treasury: Option<Pubkey>,
        new_fee_bps: Option<u16>,
        new_paused: Option<bool>,
    ) -> Result<()> {
        instructions::update_config::handler(ctx, new_authority, new_treasury, new_fee_bps, new_paused)
    }

    /// Close the config account (for migration/reinitialization).
    /// Returns rent to admin. Use with caution!
    pub fn close_config(ctx: Context<CloseConfig>) -> Result<()> {
        instructions::close_config::handler(ctx)
    }
}
