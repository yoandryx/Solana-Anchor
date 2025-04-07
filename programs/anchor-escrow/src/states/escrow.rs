// escrow.rs
use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub bump: u8,
    pub initializer: Pubkey,      // Seller’s wallet
    pub luxhub_wallet: Pubkey,    // LuxHub fee wallet
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub initializer_amount: u64,  // Total funds from sale
    pub taker_amount: u64,        // Expected funds from buyer
    pub file_cid: String,
    pub sale_price: u64,          // New field: sale price in lamports (or your chosen unit)
}

impl Escrow {
    // Define maximum length for file_cid.
    pub const MAX_CID_LENGTH: usize = 200; // adjust as needed

    // Original space: 8 (discriminator) + 8 (seed) + 1 (bump) + 32 (initializer) + 32 (luxhub_wallet) +
    // 32 (mint_a) + 32 (mint_b) + 8 (initializer_amount) + 8 (taker_amount) + (4 + MAX_CID_LENGTH)
    // New field sale_price: add 8 bytes.
    pub const INIT_SPACE: usize = 8 + 8 + 1 + 32 + 32 + 32 + 32 + 8 + 8 + (4 + Self::MAX_CID_LENGTH) + 8;
}
