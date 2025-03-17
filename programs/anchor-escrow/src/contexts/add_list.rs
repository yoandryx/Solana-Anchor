use anchor_lang::prelude::*;
use crate::states::admin_list::AdminList;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Admin list is full.")]
    AdminListFull,
}

#[derive(Accounts)]
pub struct AddAdmin<'info> {
    #[account(mut)]
    pub admin_list: Account<'info, AdminList>,
    #[account(mut, signer)]
    /// CHECK: We are only checking that the signer’s key is in the admin list.
    pub admin: AccountInfo<'info>,
    /// CHECK: The new admin to be added (only the pubkey is used).
    pub new_admin: AccountInfo<'info>,
}

pub fn add_admin(ctx: Context<AddAdmin>) -> Result<()> {
    let admin_list = &mut ctx.accounts.admin_list;
    let admin_key = ctx.accounts.admin.key;

    // Verify the signer is already an admin.
    require!(
        admin_list.admins.contains(admin_key),
        CustomError::Unauthorized
    );
    // Enforce maximum admins.
    require!(
        admin_list.admins.len() < AdminList::MAX_ADMINS,
        CustomError::AdminListFull
    );
    // Add new admin if not already present.
    if !admin_list.admins.contains(ctx.accounts.new_admin.key) {
        admin_list.admins.push(*ctx.accounts.new_admin.key);
    }
    Ok(())
}
