// instructions/initialize_config.rs
use anchor_lang::prelude::*;
use crate::InitializeConfig;
use crate::state::EscrowConfig;
use crate::errors::LuxError;

pub fn handler(
    ctx: Context<InitializeConfig>,
    authority: Pubkey,
    treasury: Pubkey,
    fee_bps: u16,
) -> Result<()> {
    require!(fee_bps <= 1000, LuxError::FeeTooHigh);

    let cfg: &mut Account<EscrowConfig> = &mut ctx.accounts.config;
    cfg.authority = authority;
    cfg.treasury = treasury;
    cfg.fee_bps = fee_bps;
    cfg.paused = false;
    cfg.bump = ctx.bumps.config;

    Ok(())
}
