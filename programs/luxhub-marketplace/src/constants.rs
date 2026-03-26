use anchor_lang::prelude::{pubkey, Pubkey};

/// Squads v4 program id (devnet/mainnet)
pub const SQUADS_V4: Pubkey = pubkey!("SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf");

// PDA Seeds
pub const ESCROW_SEED: &[u8] = b"state";
pub const CONFIG_SEED: &[u8] = b"luxhub-config";
pub const BPS_DENOM: u64 = 10_000;
