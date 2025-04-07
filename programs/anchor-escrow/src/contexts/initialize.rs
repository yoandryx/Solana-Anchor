// initialize.rs
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount}
};
use crate::states::Escrow;

#[derive(Accounts)]
#[instruction(
    seed: u64,
    initializer_amount: u64,
    taker_amount: u64,
    file_cid: String,
    luxhub_wallet: Pubkey,
    sale_price: u64
)]
pub struct Initialize<'info> {
    // The admin is the signer who pays for the account creation.
    #[account(mut)]
    pub admin: Signer<'info>,
    // The seller’s public key is passed in (not a signer).
    /// CHECK: Seller’s account is only used for its public key.
    pub seller: AccountInfo<'info>,
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    #[account(
        mut,
        // Use the seller’s public key as authority.
        associated_token::mint = mint_a,
        associated_token::authority = seller
    )]
    pub seller_ata_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = admin,
        space = Escrow::INIT_SPACE,
        seeds = [b"state", seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_escrow(
        &mut self,
        seed: u64,
        bumps: &InitializeBumps, // Assume you have a bumps struct defined
        initializer_amount: u64,
        taker_amount: u64,
        file_cid: String,
        luxhub_wallet: Pubkey,
        sale_price: u64
    ) -> Result<()> {
        // Set the escrow fields.
        self.escrow.seed = seed;
        self.escrow.bump = bumps.escrow;
        // Use the seller's public key (not the admin) as the "initializer" in escrow.
        self.escrow.initializer = self.seller.key();
        self.escrow.luxhub_wallet = luxhub_wallet;
        self.escrow.mint_a = self.mint_a.key();
        self.escrow.mint_b = self.mint_b.key();
        self.escrow.initializer_amount = initializer_amount;
        self.escrow.taker_amount = taker_amount;
        self.escrow.file_cid = file_cid;
        self.escrow.sale_price = sale_price; // sale price in lamports
        Ok(())
    }
    // Note: We remove the deposit() call so that no fund transfer is attempted during escrow creation.
}
