// instructions/add_admin.rs
use anchor_lang::prelude::*;
use crate::contexts::{AddAdmin, RemoveAdmin};
use crate::state::AdminList;

pub fn handler(ctx: Context<AddAdmin>, new_admin: Pubkey) -> Result<()> {
    let admin_list: &mut Account<AdminList> = &mut ctx.accounts.admin_list;

    admin_list.add_admin(new_admin)?;

    msg!(
        "Admin {} added by {}. Total admins: {}",
        new_admin,
        ctx.accounts.authority.key(),
        admin_list.admins.len()
    );

    Ok(())
}

pub fn remove_handler(ctx: Context<RemoveAdmin>, admin_to_remove: Pubkey) -> Result<()> {
    let admin_list: &mut Account<AdminList> = &mut ctx.accounts.admin_list;

    // Cannot remove the authority
    require!(
        admin_to_remove != admin_list.authority,
        crate::errors::LuxError::Unauthorized
    );

    admin_list.remove_admin(&admin_to_remove)?;

    msg!(
        "Admin {} removed by {}. Remaining admins: {}",
        admin_to_remove,
        ctx.accounts.authority.key(),
        admin_list.admins.len()
    );

    Ok(())
}
