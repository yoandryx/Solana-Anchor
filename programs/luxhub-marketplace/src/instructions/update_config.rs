// instructions/update_config.rs
use anchor_lang::prelude::*;
use crate::UpdateConfig;
use crate::state::EscrowConfig;
use crate::errors::LuxError;

pub fn handler(
    ctx: Context<UpdateConfig>,
    new_authority: Option<Pubkey>,
    new_treasury: Option<Pubkey>,
    new_fee_bps: Option<u16>,
    new_paused: Option<bool>,
) -> Result<()> {
    let cfg: &mut Account<EscrowConfig> = &mut ctx.accounts.config;

    // Validate fee_bps if provided (max 10% = 1000 bps)
    if let Some(fee) = new_fee_bps {
        require!(fee <= 1000, LuxError::FeeTooHigh);
    }

    // Update fields if provided
    if let Some(authority) = new_authority {
        cfg.authority = authority;
    }
    if let Some(treasury) = new_treasury {
        cfg.treasury = treasury;
    }
    if let Some(fee_bps) = new_fee_bps {
        cfg.fee_bps = fee_bps;
    }
    if let Some(paused) = new_paused {
        cfg.paused = paused;
    }

    msg!("Config updated: authority={}, treasury={}, fee_bps={}, paused={}",
         cfg.authority, cfg.treasury, cfg.fee_bps, cfg.paused);
    Ok(())
}
