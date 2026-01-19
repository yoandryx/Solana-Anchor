// contexts/admin_transfer.rs
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::state::AdminList;
use crate::constants::ADMIN_LIST_SEED;
use crate::errors::LuxError;

/// Admin transfer - move NFT from one wallet to another
/// Used for airdropping NFTs to vendors or recalling NFTs
#[derive(Accounts)]
pub struct AdminTransfer<'info> {
    /// The admin performing the transfer (must be in AdminList)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The AdminList PDA for authorization
    #[account(
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&admin.key()) @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,

    /// The NFT mint account
    #[account(
        constraint = nft_mint.mint_authority.contains(&admin.key()) @ LuxError::NotMintAuthority
    )]
    pub nft_mint: Account<'info, Mint>,

    /// The source token account (must be owned by admin or delegated to admin)
    #[account(
        mut,
        constraint = from_token_account.amount == 1 @ LuxError::InvalidTokenAmount,
        constraint = from_token_account.mint == nft_mint.key() @ LuxError::MintMismatch,
        constraint = from_token_account.owner == admin.key() @ LuxError::Unauthorized
    )]
    pub from_token_account: Account<'info, TokenAccount>,

    /// CHECK: The destination wallet
    pub to_wallet: AccountInfo<'info>,

    /// The destination token account (init_if_needed)
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = nft_mint,
        associated_token::authority = to_wallet
    )]
    pub to_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/// Admin transfer from a third-party wallet (requires delegate authority)
/// This is for transferring NFTs that vendors hold but LuxHub has authority over
#[derive(Accounts)]
pub struct AdminTransferFromVendor<'info> {
    /// The admin performing the transfer (must be in AdminList)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The AdminList PDA for authorization
    #[account(
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&admin.key()) @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,

    /// The NFT mint account
    pub nft_mint: Account<'info, Mint>,

    /// CHECK: The current owner's wallet
    pub from_wallet: AccountInfo<'info>,

    /// The source token account (vendor's account - admin must be delegate)
    #[account(
        mut,
        constraint = from_token_account.amount == 1 @ LuxError::InvalidTokenAmount,
        constraint = from_token_account.mint == nft_mint.key() @ LuxError::MintMismatch,
        constraint = from_token_account.delegate.contains(&admin.key()) @ LuxError::NotDelegate
    )]
    pub from_token_account: Account<'info, TokenAccount>,

    /// CHECK: The destination wallet
    pub to_wallet: AccountInfo<'info>,

    /// The destination token account
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = nft_mint,
        associated_token::authority = to_wallet
    )]
    pub to_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
