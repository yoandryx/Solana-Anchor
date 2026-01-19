// instructions/burn_nft.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn};
use crate::contexts::{BurnNft, BurnOwnedNft};

/// Burn an NFT owned by a third party
/// Note: This requires the token account authority to be the admin (via delegate or direct ownership)
/// In practice, use BurnOwnedNft for LuxHub-held NFTs
pub fn handler(ctx: Context<BurnNft>) -> Result<()> {
    let cpi_accounts = Burn {
        mint: ctx.accounts.nft_mint.to_account_info(),
        from: ctx.accounts.nft_token_account.to_account_info(),
        authority: ctx.accounts.token_account_authority.to_account_info(),
    };

    token::burn(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1, // Burn exactly 1 token (the NFT)
    )?;

    msg!(
        "NFT burned: mint={}, burned_by={}",
        ctx.accounts.nft_mint.key(),
        ctx.accounts.admin.key()
    );

    Ok(())
}

/// Burn an NFT that LuxHub (admin) owns directly
pub fn handler_owned(ctx: Context<BurnOwnedNft>) -> Result<()> {
    let cpi_accounts = Burn {
        mint: ctx.accounts.nft_mint.to_account_info(),
        from: ctx.accounts.admin_token_account.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };

    token::burn(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1, // Burn exactly 1 token (the NFT)
    )?;

    msg!(
        "LuxHub-owned NFT burned: mint={}, admin={}",
        ctx.accounts.nft_mint.key(),
        ctx.accounts.admin.key()
    );

    Ok(())
}
