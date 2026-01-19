// contexts/mint_nft.rs
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::state::AdminList;
use crate::constants::ADMIN_LIST_SEED;
use crate::errors::LuxError;

#[derive(Accounts)]
pub struct MintNft<'info> {
    /// The admin performing the mint (must be in AdminList)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The AdminList PDA for authorization
    #[account(
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&admin.key()) @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,

    /// CHECK: The recipient wallet that will receive the NFT
    pub recipient: AccountInfo<'info>,

    /// The NFT mint account to be created
    /// decimals = 0 for NFT, mint_authority and freeze_authority = admin (LuxHub)
    #[account(
        init,
        payer = admin,
        mint::decimals = 0,
        mint::authority = admin,
        mint::freeze_authority = admin
    )]
    pub nft_mint: Account<'info, Mint>,

    /// The recipient's associated token account for the NFT
    #[account(
        init,
        payer = admin,
        associated_token::mint = nft_mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Mint NFT to LuxHub's own wallet (for inventory holding)
#[derive(Accounts)]
pub struct MintNftToSelf<'info> {
    /// The admin performing the mint (must be in AdminList)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The AdminList PDA for authorization
    #[account(
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&admin.key()) @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,

    /// The NFT mint account to be created
    #[account(
        init,
        payer = admin,
        mint::decimals = 0,
        mint::authority = admin,
        mint::freeze_authority = admin
    )]
    pub nft_mint: Account<'info, Mint>,

    /// The admin's associated token account for the NFT
    #[account(
        init,
        payer = admin,
        associated_token::mint = nft_mint,
        associated_token::authority = admin
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
