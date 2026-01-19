// instructions/admin_transfer.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::contexts::{AdminTransfer, AdminTransferFromVendor};

/// Transfer an NFT from admin's wallet to another wallet (e.g., vendor)
pub fn handler(ctx: Context<AdminTransfer>) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };

    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1, // Transfer exactly 1 token (the NFT)
    )?;

    msg!(
        "NFT transferred: mint={}, from={}, to={}, by={}",
        ctx.accounts.nft_mint.key(),
        ctx.accounts.from_token_account.key(),
        ctx.accounts.to_wallet.key(),
        ctx.accounts.admin.key()
    );

    Ok(())
}

/// Transfer an NFT from a vendor's wallet (using delegate authority)
pub fn handler_from_vendor(ctx: Context<AdminTransferFromVendor>) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(), // Admin as delegate
    };

    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1, // Transfer exactly 1 token (the NFT)
    )?;

    msg!(
        "NFT recalled from vendor: mint={}, from_vendor={}, to={}, by={}",
        ctx.accounts.nft_mint.key(),
        ctx.accounts.from_wallet.key(),
        ctx.accounts.to_wallet.key(),
        ctx.accounts.admin.key()
    );

    Ok(())
}
