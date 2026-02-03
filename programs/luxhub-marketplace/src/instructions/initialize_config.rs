// instructions/initialize_config.rs
use anchor_lang::prelude::*;
use crate::InitializeConfig;
use crate::state::EscrowConfig;

pub fn handler(
    ctx: Context<InitializeConfig>,
    authority: Pubkey,
    treasury: Pubkey,
    fee_bps: u16,
) -> Result<()> {
    let cfg: &mut Account<EscrowConfig> = &mut ctx.accounts.config;
    cfg.authority = authority;
    cfg.treasury = treasury;
    cfg.fee_bps = fee_bps;
    cfg.paused = false;
    cfg.bump = ctx.bumps.config;

    msg!("Config initialized: authority={}, treasury={}, fee_bps={}",
         authority, treasury, fee_bps);
    Ok(())
}
