use anchor_lang::prelude::*;
use crate::states::admin_list::AdminList;

#[derive(Accounts)]
pub struct InitAdminList<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 4 + (AdminList::MAX_ADMINS * 32),  // ✅ Correct space calculation
        seeds = [b"admin_list"],  // ✅ Ensure seed matches front-end derivation
        bump
    )]
    pub admin_list: Account<'info, AdminList>,  // ✅ This refers to the on-chain AdminList account
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_admin_list(ctx: Context<InitAdminList>) -> Result<()> {
    let admin_list = &mut ctx.accounts.admin_list;
    
    // ✅ Correct: Initializes the vector with the first admin (payer)
    admin_list.admins = vec![ctx.accounts.payer.key()];
    
    Ok(())
}
