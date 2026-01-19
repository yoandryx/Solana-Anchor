// state/escrow.rs
use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub bump: u8,
    pub initializer: Pubkey,
    pub luxhub_wallet: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
    pub file_cid: String,
    pub sale_price: u64,
    pub is_completed: bool,
    pub buyer: Pubkey,
}
impl Escrow {
    pub const MAX_CID_LENGTH: usize = 200;
    pub const SIZE: usize = 8 + 1 + 32 + 32 + 32 + 32 + 8 + 8 + (4 + Self::MAX_CID_LENGTH) + 8 + 1 + 32;
}
