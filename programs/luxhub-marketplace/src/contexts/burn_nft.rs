// contexts/burn_nft.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::AdminList;
use crate::constants::ADMIN_LIST_SEED;
use crate::errors::LuxError;

/// Burn an NFT - permanently destroy it
/// Used when owner is unreachable or asset is no longer valid
#[derive(Accounts)]
pub struct BurnNft<'info> {
    /// The admin performing the burn (must be in AdminList)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The AdminList PDA for authorization
    #[account(
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&admin.key()) @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,

    /// The NFT mint account to be burned
    /// Admin must be the mint authority
    #[account(
        mut,
        constraint = nft_mint.mint_authority.contains(&admin.key()) @ LuxError::NotMintAuthority
    )]
    pub nft_mint: Account<'info, Mint>,

    /// The token account holding the NFT
    /// Must have exactly 1 token to burn
    #[account(
        mut,
        constraint = nft_token_account.amount == 1 @ LuxError::InvalidTokenAmount,
        constraint = nft_token_account.mint == nft_mint.key() @ LuxError::MintMismatch
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    /// CHECK: The current owner of the token account (needed for CPI)
    pub token_account_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

/// Burn an NFT that LuxHub owns (admin is the token authority)
#[derive(Accounts)]
pub struct BurnOwnedNft<'info> {
    /// The admin performing the burn (must be in AdminList and owns the NFT)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The AdminList PDA for authorization
    #[account(
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&admin.key()) @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,

    /// The NFT mint account to be burned
    #[account(
        mut,
        constraint = nft_mint.mint_authority.contains(&admin.key()) @ LuxError::NotMintAuthority
    )]
    pub nft_mint: Account<'info, Mint>,

    /// The admin's token account holding the NFT
    #[account(
        mut,
        constraint = admin_token_account.amount == 1 @ LuxError::InvalidTokenAmount,
        constraint = admin_token_account.mint == nft_mint.key() @ LuxError::MintMismatch,
        constraint = admin_token_account.owner == admin.key() @ LuxError::Unauthorized
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
