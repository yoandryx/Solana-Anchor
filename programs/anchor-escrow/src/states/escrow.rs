use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub bump: u8,
    pub initializer: Pubkey,      // Seller’s wallet
    pub luxhub_wallet: Pubkey,    // New: LuxHub escrow wallet
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub initializer_amount: u64,  // Total funds from sale
    pub taker_amount: u64,
    pub file_cid: String,
}

impl Escrow {
    // Define a maximum length for your file CID string.
    pub const MAX_CID_LENGTH: usize = 200; // Adjust as needed
    // Updated space calculation:
    // 8 (discriminator) + 8 (seed) + 1 (bump) + 32 (initializer) + 32 (luxhub_wallet) +
    // 32 (mint_a) + 32 (mint_b) + 8 (initializer_amount) + 8 (taker_amount) +
    // 4 (string length) + MAX_CID_LENGTH
    pub const INIT_SPACE: usize = 8 + 8 + 1 + 32 + 32 + 32 + 32 + 8 + 8 + (4 + Self::MAX_CID_LENGTH);
}
