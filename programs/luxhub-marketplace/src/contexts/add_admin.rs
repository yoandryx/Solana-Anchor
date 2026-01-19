// contexts/add_admin.rs
use anchor_lang::prelude::*;
use crate::state::AdminList;
use crate::constants::ADMIN_LIST_SEED;
use crate::errors::LuxError;

#[derive(Accounts)]
pub struct AddAdmin<'info> {
    /// The authority (must be current authority or existing admin)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The AdminList PDA account
    #[account(
        mut,
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.is_admin(&authority.key()) || admin_list.authority == authority.key() @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,
}

#[derive(Accounts)]
pub struct RemoveAdmin<'info> {
    /// The authority (must be the list authority)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The AdminList PDA account
    #[account(
        mut,
        seeds = [ADMIN_LIST_SEED],
        bump = admin_list.bump,
        constraint = admin_list.authority == authority.key() @ LuxError::Unauthorized
    )]
    pub admin_list: Account<'info, AdminList>,
}
