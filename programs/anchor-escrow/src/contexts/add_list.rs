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
    /// CHECK: The admin requesting to add a new admin.
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: The new admin to be added (only the pubkey is used).
    pub new_admin: AccountInfo<'info>,
}

pub fn add_admin(ctx: Context<AddAdmin>) -> Result<()> {
    let admin_list = &mut ctx.accounts.admin_list;
  
    // ✅ Verify the signer is already an admin.
    require!(
        admin_list.admins.contains(&ctx.accounts.admin.key()),
        CustomError::Unauthorized
    );

    // ✅ Enforce maximum number of admins.
    require!(
        admin_list.admins.len() < AdminList::MAX_ADMINS,
        CustomError::AdminListFull
    );

    // ✅ Add new admin only if not already present.
    if !admin_list.admins.contains(&ctx.accounts.new_admin.key()) {
        admin_list.admins.push(ctx.accounts.new_admin.key());
    }

    Ok(())
}
