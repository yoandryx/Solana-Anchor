// state/config.rs
use anchor_lang::prelude::*;

#[account]
pub struct EscrowConfig {
    /// The Squads multisig that controls this config
    pub authority: Pubkey,
    /// The treasury vault where fees are sent (Squads Vault PDA)
    pub treasury: Pubkey,
    /// Fee in basis points (300 = 3%)
    pub fee_bps: u16,
    /// Emergency pause flag
    pub paused: bool,
    /// Bump seed for PDA derivation
    pub bump: u8,
}

impl EscrowConfig {
    // 32 (authority) + 32 (treasury) + 2 (fee_bps) + 1 (paused) + 1 (bump) = 68
    pub const SIZE: usize = 32 + 32 + 2 + 1 + 1;
}
