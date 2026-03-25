// instructions/confirm_delivery.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::ConfirmDelivery;
use crate::constants::BPS_DENOM;
use anchor_lang::solana_program::sysvar::instructions as ix_sysvar;
use crate::constants::SQUADS_V4_PUBKEY;
use crate::errors::LuxError;
use crate::utils::squads_gate::enforce_squads_cpi;

pub fn handler(ctx: Context<ConfirmDelivery>) -> Result<()> {
    let config = &ctx.accounts.config;

    // Check if protocol is paused
    require!(!config.paused, LuxError::ProtocolPaused);

    // --- Squads CPI origin check (CHECK CURRENT top-level ix) ---
    let ix_info = &ctx.accounts.instructions_sysvar;
    let current_index = ix_sysvar::load_current_index_checked(ix_info)? as usize;
    let cur_ix = ix_sysvar::load_instruction_at_checked(current_index, ix_info)?;
    require_keys_eq!(
        cur_ix.program_id,
        SQUADS_V4_PUBKEY,
        LuxError::NotCalledBySquads
    );

    // Squads gate - verify authority matches config
    require_keys_eq!(ctx.accounts.authority.key(), config.authority, LuxError::Unauthorized);
    enforce_squads_cpi(&ctx.accounts.instructions_sysvar, &config.authority)?;

    let escrow = &mut ctx.accounts.escrow;
    // is_completed and buyer checks are enforced in account constraints
    require!(ctx.accounts.wsol_vault.amount >= escrow.sale_price, LuxError::Unauthorized);

    let signer_seeds = &[b"state", &escrow.seed.to_le_bytes()[..], &[escrow.bump]];
    let cpi_signers = &[&signer_seeds[..]];

    // NFT -> buyer
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_vault.to_account_info(),
                to: ctx.accounts.buyer_nft_ata.to_account_info(),
                authority: escrow.to_account_info(),
            },
            cpi_signers,
        ),
        escrow.initializer_amount,
    )?;

    // Calculate fee split using on-chain config fee_bps
    let fee_bps = config.fee_bps as u64;
    let seller_bps = BPS_DENOM - fee_bps;

    let seller_share = escrow.sale_price.checked_mul(seller_bps).ok_or(LuxError::MathOverflow)?
        .checked_div(BPS_DENOM).ok_or(LuxError::MathOverflow)?;
    let fee_share = escrow.sale_price.checked_mul(fee_bps).ok_or(LuxError::MathOverflow)?
        .checked_div(BPS_DENOM).ok_or(LuxError::MathOverflow)?;

    // Fee -> treasury (config.treasury)
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.wsol_vault.to_account_info(),
                to: ctx.accounts.luxhub_fee_ata.to_account_info(),
                authority: escrow.to_account_info(),
            },
            cpi_signers,
        ),
        fee_share,
    )?;

    // Seller share
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.wsol_vault.to_account_info(),
                to: ctx.accounts.seller_funds_ata.to_account_info(),
                authority: escrow.to_account_info(),
            },
            cpi_signers,
        ),
        seller_share,
    )?;

    escrow.is_completed = true;
    msg!("Delivery confirmed: seller_share={}, fee_share={} ({}bps)",
         seller_share, fee_share, fee_bps);
    Ok(())
}
