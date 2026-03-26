// instructions/cancel_escrow.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Transfer};

use crate::CancelEscrow;
use crate::constants::ESCROW_SEED;

/// Cancel an escrow listing and return the NFT to the seller.
/// Only callable by the original seller before a buyer deposits funds.
pub fn handler(ctx: Context<CancelEscrow>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    // Build signer seeds for the escrow PDA
    let seeds = &[
        ESCROW_SEED,
        &escrow.seed.to_le_bytes()[..],
        &[escrow.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // Transfer NFT back to seller
    let cpi_accounts = Transfer {
        from: ctx.accounts.nft_vault.to_account_info(),
        to: ctx.accounts.seller_nft_ata.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, 1)?;

    // Close NFT vault and return rent to seller
    let close_nft_vault = CloseAccount {
        account: ctx.accounts.nft_vault.to_account_info(),
        destination: ctx.accounts.seller.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        close_nft_vault,
        signer_seeds,
    ))?;

    // Close WSOL vault and return rent to seller
    let close_wsol_vault = CloseAccount {
        account: ctx.accounts.wsol_vault.to_account_info(),
        destination: ctx.accounts.seller.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        close_wsol_vault,
        signer_seeds,
    ))?;

    Ok(())
}
