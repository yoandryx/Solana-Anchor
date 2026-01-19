// contexts/admin_only_example.rs
use anchor_lang::prelude::*;
use crate::state::EscrowConfig;
use crate::constants::CONFIG_SEED;

#[derive(Accounts)]
pub struct AdminOnlyExample<'info> {
    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, EscrowConfig>,

    /// CHECK: Verified in handler against `config.squads_authority`
    pub authority: AccountInfo<'info>,
    /// CHECK: Read-only sysvar; we only inspect its bytes
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
}
