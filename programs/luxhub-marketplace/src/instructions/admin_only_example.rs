// instructions/admin_only_example.rs
use anchor_lang::prelude::*;
use crate::contexts;
use crate::errors::LuxError;
use crate::utils::squads_gate::enforce_squads_cpi;

pub fn handler(ctx: Context<contexts::AdminOnlyExample>) -> Result<()> {
    require_keys_eq!(ctx.accounts.authority.key(), ctx.accounts.config.squads_authority, LuxError::Unauthorized);
    enforce_squads_cpi(&ctx.accounts.instructions_sysvar, &ctx.accounts.config.squads_authority)?;
    // privileged work ...
    Ok(())
}
