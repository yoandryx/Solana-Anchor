// admin_list.rs
use anchor_lang::prelude::*;

#[account]
pub struct AdminList {
    // A simple vector of admin pubkeys.
    pub admins: Vec<Pubkey>,
}

// A constant to define maximum admins (optional, if you wish to limit the size).
impl AdminList {
    pub const MAX_ADMINS: usize = 10; // adjust as needed
}
