use anchor_lang::prelude::*;

#[account]
pub struct EscrowConfig {
    /// The current LuxHub wallet address used by new escrows.
    pub luxhub_wallet: Pubkey,
}

impl EscrowConfig {
    pub const SIZE: usize = 8 + 32; // discriminator (8 bytes) + Pubkey (32 bytes)
}
