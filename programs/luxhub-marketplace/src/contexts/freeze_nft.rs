// contexts/freeze_nft.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::AdminList;
use crate::constants::ADMIN_LIST_SEED;
use crate::errors::LuxError;

/// Freeze an NFT - prevent all transfers
/// Used for suspicious activity or pending investigation
#[derive(Accounts)]
pub struct FreezeNft<'info> {
    /// The admin performing the freeze (must be in AdminList)
    pub admin: Signer<'info>,

    /// The AdminList PDA for authorization
    #[account(
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&admin.key()) @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,

    /// The NFT mint account
    /// Admin must be the freeze authority
    #[account(
        constraint = nft_mint.freeze_authority.contains(&admin.key()) @ LuxError::NotFreezeAuthority
    )]
    pub nft_mint: Account<'info, Mint>,

    /// The token account to freeze
    #[account(
        mut,
        constraint = nft_token_account.mint == nft_mint.key() @ LuxError::MintMismatch
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

/// Thaw (unfreeze) an NFT - allow transfers again
#[derive(Accounts)]
pub struct ThawNft<'info> {
    /// The admin performing the thaw (must be in AdminList)
    pub admin: Signer<'info>,

    /// The AdminList PDA for authorization
    #[account(
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&admin.key()) @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,

    /// The NFT mint account
    /// Admin must be the freeze authority
    #[account(
        constraint = nft_mint.freeze_authority.contains(&admin.key()) @ LuxError::NotFreezeAuthority
    )]
    pub nft_mint: Account<'info, Mint>,

    /// The token account to thaw
    #[account(
        mut,
        constraint = nft_token_account.mint == nft_mint.key() @ LuxError::MintMismatch
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
