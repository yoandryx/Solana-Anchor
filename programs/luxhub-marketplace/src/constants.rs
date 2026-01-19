use anchor_lang::prelude::{pubkey, Pubkey};

/// Squads v4 program id (devnet/mainnet)
pub const SQUADS_V4_PUBKEY: Pubkey = pubkey!("SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf");
pub const SQUADS_V4: Pubkey = pubkey!("SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf");

// PDA Seeds
pub const ESCROW_SEED: &[u8] = b"state";
pub const CONFIG_SEED: &[u8] = b"luxhub-config";
pub const ADMIN_LIST_SEED: &[u8] = b"admin-list";

// App-tunable split for escrow
pub const SELLER_BPS: u64 = 9500; // 95%
pub const FEE_BPS: u64 = 500;     // 5%
pub const BPS_DENOM: u64 = 10_000;
