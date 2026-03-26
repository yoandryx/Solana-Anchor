// instructions/close_config.rs
// Closes the config account to allow reinitialization with new structure
// Anchor's `close = admin` constraint handles lamport transfer, data zeroing, and discriminator clearing.
use anchor_lang::prelude::*;
use crate::CloseConfig;

pub fn handler(_ctx: Context<CloseConfig>) -> Result<()> {
    Ok(())
}
