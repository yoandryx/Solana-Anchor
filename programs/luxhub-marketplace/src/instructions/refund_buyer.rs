// instructions/refund_buyer.rs
// Refunds buyer's funds (USDC) and returns NFT to seller
// Gated by Squads CPI (same pattern as confirm_delivery)
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, CloseAccount};
use crate::RefundBuyer;
use crate::constants::ESCROW_SEED;
use crate::errors::LuxError;
use crate::utils::squads_gate::enforce_squads_cpi;

pub fn handler(ctx: Context<RefundBuyer>) -> Result<()> {
    let config = &ctx.accounts.config;

    // Check if protocol is paused
    require!(!config.paused, LuxError::ProtocolPaused);

    // Squads gate - verify authority matches config and CPI origin
    require_keys_eq!(ctx.accounts.authority.key(), config.authority, LuxError::Unauthorized);
    enforce_squads_cpi(&ctx.accounts.instructions_sysvar, &config.authority)?;

    let escrow = &mut ctx.accounts.escrow;
    // buyer != default and !is_completed checks are enforced in account constraints

    let signer_seeds = &[ESCROW_SEED, &escrow.seed.to_le_bytes()[..], &[escrow.bump]];
    let cpi_signers = &[&signer_seeds[..]];

    // 1. Return all funds from vault -> buyer's ATA
    let vault_amount = ctx.accounts.funds_vault.amount;
    if vault_amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.funds_vault.to_account_info(),
                    to: ctx.accounts.buyer_funds_ata.to_account_info(),
                    authority: escrow.to_account_info(),
                },
                cpi_signers,
            ),
            vault_amount,
        )?;
    }

    // 2. Return NFT from vault -> seller's ATA
    let nft_amount = ctx.accounts.nft_vault.amount;
    if nft_amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.nft_vault.to_account_info(),
                    to: ctx.accounts.seller_nft_ata.to_account_info(),
                    authority: escrow.to_account_info(),
                },
                cpi_signers,
            ),
            nft_amount,
        )?;
    }

    // 3. Close vault token accounts (return rent to buyer)
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.funds_vault.to_account_info(),
            destination: ctx.accounts.buyer_account.to_account_info(),
            authority: escrow.to_account_info(),
        },
        cpi_signers,
    ))?;

    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.nft_vault.to_account_info(),
            destination: ctx.accounts.buyer_account.to_account_info(),
            authority: escrow.to_account_info(),
        },
        cpi_signers,
    ))?;

    escrow.is_completed = true;
    Ok(())
}
