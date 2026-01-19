// src/contexts/initialize.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{Escrow, EscrowConfig};
use crate::constants::{CONFIG_SEED, ESCROW_SEED};

#[derive(Accounts)]
#[instruction(seed: u64)] // Instruction argument for PDA seed
pub struct Initialize<'info> {
    /// Payer for PDA initialization (admin, typically the transaction initiator)
    #[account(mut)]
    pub admin: Signer<'info>,

    /// Seller who owns the NFT
    #[account(mut)]
    pub seller: Signer<'info>,

    /// Config PDA, pre-created via initialize_config instruction
    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, EscrowConfig>,

    /// Funds mint (e.g., wSOL-like SPL token)
    pub mint_a: Account<'info, Mint>,

    /// NFT mint
    pub mint_b: Account<'info, Mint>,

    /// Seller’s token account for mint_a (e.g., wSOL)
    #[account(
        mut,
        constraint = seller_ata_a.mint == mint_a.key() @ ErrorCode::InvalidMint,
        constraint = seller_ata_a.owner == seller.key() @ ErrorCode::InvalidOwner
    )]
    pub seller_ata_a: Account<'info, TokenAccount>,

    /// Seller’s token account for mint_b (e.g., NFT)
    #[account(
        mut,
        constraint = seller_ata_b.mint == mint_b.key() @ ErrorCode::InvalidMint,
        constraint = seller_ata_b.owner == seller.key() @ ErrorCode::InvalidOwner,
        constraint = seller_ata_b.amount == 1 @ ErrorCode::InvalidNFTAmount // Ensure NFT amount is 1
    )]
    pub seller_ata_b: Account<'info, TokenAccount>,

    /// Escrow PDA account, derived from the instruction arg `seed`
    #[account(
        init,
        payer = admin,
        space = 8 + Escrow::SIZE,
        seeds = [ESCROW_SEED, &seed.to_le_bytes()[..]], // Explicitly slice to ensure correct bytes
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// Escrow vault for NFT (mint_b)
    #[account(
        init,
        payer = admin,
        token::mint = mint_b,
        token::authority = escrow
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    /// Escrow vault for funds (mint_a, e.g., wSOL)
    #[account(
        init,
        payer = admin,
        token::mint = mint_a,
        token::authority = escrow
    )]
    pub wsol_vault: Account<'info, TokenAccount>,

    /// SPL Token Program
    pub token_program: Program<'info, Token>,

    /// Solana System Program
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Token account mint does not match the provided mint.")]
    InvalidMint,
    #[msg("Token account owner does not match the seller.")]
    InvalidOwner,
    #[msg("NFT token account must hold exactly one token.")]
    InvalidNFTAmount,
}