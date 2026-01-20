// instructions/initialize_config.rs
use anchor_lang::prelude::*;
use crate::InitializeConfig;
use crate::state::EscrowConfig;

pub fn handler(
    ctx: Context<InitializeConfig>,
    squads_multisig: Pubkey,
    squads_authority: Pubkey,
) -> Result<()> {
    let cfg: &mut Account<EscrowConfig> = &mut ctx.accounts.config;
    cfg.squads_multisig = squads_multisig;
    cfg.squads_authority = squads_authority;
    Ok(())
}
