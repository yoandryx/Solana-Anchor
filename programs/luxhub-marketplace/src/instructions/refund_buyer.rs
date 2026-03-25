// instructions/refund_buyer.rs
// Refunds buyer's funds (USDC) and returns NFT to seller
// Gated by Squads CPI (same pattern as confirm_delivery)
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, CloseAccount};
use crate::RefundBuyer;
use anchor_lang::solana_program::sysvar::instructions as ix_sysvar;
use crate::constants::SQUADS_V4_PUBKEY;
use crate::errors::LuxError;
use crate::utils::squads_gate::enforce_squads_cpi;

pub fn handler(ctx: Context<RefundBuyer>) -> Result<()> {
    let config = &ctx.accounts.config;

    // Check if protocol is paused
    require!(!config.paused, LuxError::ProtocolPaused);

    // --- Squads CPI origin check ---
    let ix_info = &ctx.accounts.instructions_sysvar;
    let current_index = ix_sysvar::load_current_index_checked(ix_info)? as usize;
    let cur_ix = ix_sysvar::load_instruction_at_checked(current_index, ix_info)?;
    require_keys_eq!(
        cur_ix.program_id,
        SQUADS_V4_PUBKEY,
        LuxError::NotCalledBySquads
    );

    // Verify authority matches config
    require_keys_eq!(ctx.accounts.authority.key(), config.authority, LuxError::Unauthorized);
    enforce_squads_cpi(&ctx.accounts.instructions_sysvar, &config.authority)?;

    let escrow = &mut ctx.accounts.escrow;

    // Must have a buyer (funded) and not already completed
    require!(escrow.buyer != Pubkey::default(), LuxError::Unauthorized);
    require!(!escrow.is_completed, LuxError::EscrowAlreadyCompleted);

    let signer_seeds = &[b"state", &escrow.seed.to_le_bytes()[..], &[escrow.bump]];
    let cpi_signers = &[&signer_seeds[..]];

    // 1. Return all funds from vault → buyer's ATA
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

    // 2. Return NFT from vault → seller's ATA
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

    // 3. Close vault token accounts (return rent to authority)
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.funds_vault.to_account_info(),
            destination: ctx.accounts.authority.to_account_info(),
            authority: escrow.to_account_info(),
        },
        cpi_signers,
    ))?;

    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.nft_vault.to_account_info(),
            destination: ctx.accounts.authority.to_account_info(),
            authority: escrow.to_account_info(),
        },
        cpi_signers,
    ))?;

    escrow.is_completed = true;
    msg!("Refund completed: {} tokens returned to buyer", vault_amount);
    Ok(())
}
