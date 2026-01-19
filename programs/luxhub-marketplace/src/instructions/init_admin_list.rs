// instructions/init_admin_list.rs
use anchor_lang::prelude::*;
use crate::contexts::InitAdminList;
use crate::state::AdminList;

pub fn handler(ctx: Context<InitAdminList>) -> Result<()> {
    let admin_list: &mut Account<AdminList> = &mut ctx.accounts.admin_list;

    // Set the bump seed
    admin_list.bump = ctx.bumps.admin_list;

    // Set the authority (the one who created the list)
    admin_list.authority = ctx.accounts.authority.key();

    // Add the authority as the first admin
    admin_list.admins = vec![ctx.accounts.authority.key()];

    msg!("AdminList initialized with authority: {}", ctx.accounts.authority.key());

    Ok(())
}
