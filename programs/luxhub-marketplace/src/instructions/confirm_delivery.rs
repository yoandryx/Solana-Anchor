// instructions/confirm_delivery.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, CloseAccount};
use crate::ConfirmDelivery;
use crate::constants::{BPS_DENOM, ESCROW_SEED};
use crate::errors::LuxError;
use crate::utils::squads_gate::enforce_squads_cpi;

pub fn handler(ctx: Context<ConfirmDelivery>) -> Result<()> {
    let config = &ctx.accounts.config;

    // Check if protocol is paused
    require!(!config.paused, LuxError::ProtocolPaused);

    // Squads gate - verify authority matches config and CPI origin
    require_keys_eq!(ctx.accounts.authority.key(), config.authority, LuxError::Unauthorized);
    enforce_squads_cpi(&ctx.accounts.instructions_sysvar, &config.authority)?;

    let escrow = &mut ctx.accounts.escrow;
    // is_completed and buyer checks are enforced in account constraints
    require!(ctx.accounts.wsol_vault.amount >= escrow.sale_price, LuxError::Unauthorized);

    let signer_seeds = &[ESCROW_SEED, &escrow.seed.to_le_bytes()[..], &[escrow.bump]];
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

    let fee_share = escrow.sale_price.checked_mul(fee_bps).ok_or(LuxError::MathOverflow)?
        .checked_div(BPS_DENOM).ok_or(LuxError::MathOverflow)?;
    let seller_share = escrow.sale_price.checked_sub(fee_share).ok_or(LuxError::MathOverflow)?;

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

    // Close vault token accounts (return rent to seller)
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.nft_vault.to_account_info(),
            destination: ctx.accounts.seller.to_account_info(),
            authority: escrow.to_account_info(),
        },
        cpi_signers,
    ))?;

    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.wsol_vault.to_account_info(),
            destination: ctx.accounts.seller.to_account_info(),
            authority: escrow.to_account_info(),
        },
        cpi_signers,
    ))?;

    escrow.is_completed = true;
    Ok(())
}
