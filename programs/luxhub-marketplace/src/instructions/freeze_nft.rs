// instructions/freeze_nft.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, FreezeAccount, ThawAccount};
use crate::contexts::{FreezeNft, ThawNft};

/// Freeze an NFT token account - prevents all transfers
pub fn handler(ctx: Context<FreezeNft>) -> Result<()> {
    let cpi_accounts = FreezeAccount {
        account: ctx.accounts.nft_token_account.to_account_info(),
        mint: ctx.accounts.nft_mint.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };

    token::freeze_account(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
    )?;

    msg!(
        "NFT frozen: mint={}, token_account={}, frozen_by={}",
        ctx.accounts.nft_mint.key(),
        ctx.accounts.nft_token_account.key(),
        ctx.accounts.admin.key()
    );

    Ok(())
}

/// Thaw an NFT token account - allows transfers again
pub fn thaw_handler(ctx: Context<ThawNft>) -> Result<()> {
    let cpi_accounts = ThawAccount {
        account: ctx.accounts.nft_token_account.to_account_info(),
        mint: ctx.accounts.nft_mint.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };

    token::thaw_account(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
    )?;

    msg!(
        "NFT thawed: mint={}, token_account={}, thawed_by={}",
        ctx.accounts.nft_mint.key(),
        ctx.accounts.nft_token_account.key(),
        ctx.accounts.admin.key()
    );

    Ok(())
}
