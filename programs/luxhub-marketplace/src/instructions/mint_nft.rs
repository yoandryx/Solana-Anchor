// instructions/mint_nft.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use crate::contexts::{MintNft, MintNftToSelf};

/// Mint an NFT to a specified recipient (vendor wallet)
pub fn handler(ctx: Context<MintNft>) -> Result<()> {
    // Mint exactly 1 token (NFT) to the recipient's associated token account
    let cpi_accounts = MintTo {
        mint: ctx.accounts.nft_mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };

    token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1, // Exactly 1 token for NFT
    )?;

    msg!(
        "NFT minted: mint={}, recipient={}, minted_by={}",
        ctx.accounts.nft_mint.key(),
        ctx.accounts.recipient.key(),
        ctx.accounts.admin.key()
    );

    Ok(())
}

/// Mint an NFT to the admin's own wallet (LuxHub inventory)
pub fn handler_to_self(ctx: Context<MintNftToSelf>) -> Result<()> {
    // Mint exactly 1 token (NFT) to the admin's associated token account
    let cpi_accounts = MintTo {
        mint: ctx.accounts.nft_mint.to_account_info(),
        to: ctx.accounts.admin_token_account.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };

    token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1, // Exactly 1 token for NFT
    )?;

    msg!(
        "NFT minted to LuxHub inventory: mint={}, admin={}",
        ctx.accounts.nft_mint.key(),
        ctx.accounts.admin.key()
    );

    Ok(())
}
