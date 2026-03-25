// instructions/close_config.rs
// Closes the config account to allow reinitialization with new structure
use anchor_lang::prelude::*;
use crate::CloseConfig;
use crate::errors::LuxError;

pub fn handler(ctx: Context<CloseConfig>) -> Result<()> {
    let config_info = &ctx.accounts.config;
    let admin_info = &ctx.accounts.admin;

    // Authority check: read the authority pubkey from config account data
    // Layout: 8-byte discriminator + 32-byte authority pubkey
    let data = config_info.try_borrow_data()?;
    require!(data.len() >= 40, LuxError::Unauthorized);
    let authority = Pubkey::try_from(&data[8..40]).map_err(|_| LuxError::Unauthorized)?;
    require_keys_eq!(admin_info.key(), authority, LuxError::Unauthorized);
    drop(data);

    // Transfer all lamports from config to admin
    let config_lamports = config_info.lamports();
    **config_info.try_borrow_mut_lamports()? = 0;
    **admin_info.try_borrow_mut_lamports()? = admin_info
        .lamports()
        .checked_add(config_lamports)
        .unwrap();

    // Zero out the data
    let mut data = config_info.try_borrow_mut_data()?;
    for byte in data.iter_mut() {
        *byte = 0;
    }

    msg!("Config account closed. Rent returned: {} lamports", config_lamports);
    Ok(())
}
