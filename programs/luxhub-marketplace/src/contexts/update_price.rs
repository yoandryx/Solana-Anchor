// src/contexts/update_price.rs
use anchor_lang::prelude::*;
use crate::state::Escrow;
use crate::constants::ESCROW_SEED;

/// Context for updating the sale price of an escrow listing.
/// Only the seller (initializer) can update the price before a buyer is assigned.
#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    /// The seller who created the escrow - must be the original initializer
    #[account(mut)]
    pub seller: Signer<'info>,

    /// The escrow account to update
    #[account(
        mut,
        seeds = [ESCROW_SEED, &escrow.seed.to_le_bytes()[..]],
        bump = escrow.bump,
        // Ensure the signer is the original seller
        constraint = escrow.initializer == seller.key() @ crate::errors::LuxError::NotSeller,
        // Ensure escrow is not completed
        constraint = !escrow.is_completed @ crate::errors::LuxError::EscrowAlreadyCompleted,
        // Ensure no buyer assigned yet (buyer should be default pubkey)
        constraint = escrow.buyer == Pubkey::default() @ crate::errors::LuxError::EscrowHasBuyer,
    )]
    pub escrow: Account<'info, Escrow>,
}
