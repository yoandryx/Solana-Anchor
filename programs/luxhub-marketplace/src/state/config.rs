// state/config.rs
use anchor_lang::prelude::*;

#[account]
pub struct EscrowConfig {
    pub squads_multisig: Pubkey,
    pub squads_authority: Pubkey,
}
impl EscrowConfig {
    pub const SIZE: usize = 32 + 32;
}
