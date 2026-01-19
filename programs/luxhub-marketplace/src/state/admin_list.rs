// state/admin_list.rs
use anchor_lang::prelude::*;

/// Maximum number of admins that can be stored in the AdminList.
/// 10 admins * 32 bytes each = 320 bytes for the vector data
pub const MAX_ADMINS: usize = 10;

#[account]
pub struct AdminList {
    /// The list of authorized admin public keys
    pub admins: Vec<Pubkey>,
    /// PDA bump seed
    pub bump: u8,
    /// The authority that can add/remove admins (typically the first admin or a multisig)
    pub authority: Pubkey,
}

impl AdminList {
    /// Calculate space needed for AdminList account
    /// 8 (discriminator) + 4 (vec length) + (32 * MAX_ADMINS) + 1 (bump) + 32 (authority)
    pub const SIZE: usize = 8 + 4 + (32 * MAX_ADMINS) + 1 + 32;

    /// Check if a pubkey is in the admin list
    pub fn is_admin(&self, pubkey: &Pubkey) -> bool {
        self.admins.contains(pubkey)
    }

    /// Add an admin to the list
    pub fn add_admin(&mut self, pubkey: Pubkey) -> Result<()> {
        require!(
            self.admins.len() < MAX_ADMINS,
            AdminListError::MaxAdminsReached
        );
        require!(
            !self.admins.contains(&pubkey),
            AdminListError::AdminAlreadyExists
        );
        self.admins.push(pubkey);
        Ok(())
    }

    /// Remove an admin from the list
    pub fn remove_admin(&mut self, pubkey: &Pubkey) -> Result<()> {
        let index = self
            .admins
            .iter()
            .position(|&p| p == *pubkey)
            .ok_or(AdminListError::AdminNotFound)?;
        self.admins.remove(index);
        Ok(())
    }
}

#[error_code]
pub enum AdminListError {
    #[msg("Maximum number of admins reached.")]
    MaxAdminsReached,
    #[msg("Admin already exists in the list.")]
    AdminAlreadyExists,
    #[msg("Admin not found in the list.")]
    AdminNotFound,
}
