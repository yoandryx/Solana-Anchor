// contexts/initialize_config.rs
use anchor_lang::prelude::*;
use crate::state::EscrowConfig;
use crate::constants::CONFIG_SEED;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + EscrowConfig::SIZE,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, EscrowConfig>,

    pub system_program: Program<'info, System>,
}
