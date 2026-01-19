// contexts/init_admin_list.rs
use anchor_lang::prelude::*;
use crate::state::AdminList;
use crate::constants::ADMIN_LIST_SEED;

#[derive(Accounts)]
pub struct InitAdminList<'info> {
    /// The initial authority who will be added as the first admin
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The AdminList PDA account to be initialized
    #[account(
        init,
        payer = authority,
        space = AdminList::SIZE,
        seeds = [ADMIN_LIST_SEED],
        bump
    )]
    pub admin_list: Account<'info, AdminList>,

    pub system_program: Program<'info, System>,
}
