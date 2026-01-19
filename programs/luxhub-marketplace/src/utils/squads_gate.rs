use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions as ix_sysvar;

use crate::constants::SQUADS_V4;
use crate::errors::LuxError;

pub fn enforce_squads_cpi(
    instructions_sysvar: &AccountInfo,
    expected_authority: &Pubkey,
) -> Result<()> {
    let current_index = ix_sysvar::load_current_index_checked(instructions_sysvar)? as usize;
    require!(current_index > 0, LuxError::NotCalledBySquads);

    let prev_ix = ix_sysvar::load_instruction_at_checked(current_index - 1, instructions_sysvar)?;
    require_keys_eq!(prev_ix.program_id, SQUADS_V4, LuxError::NotCalledBySquads);

    let saw_auth_signer = prev_ix
        .accounts
        .iter()
        .any(|m| m.pubkey == *expected_authority && m.is_signer);
    require!(saw_auth_signer, LuxError::NotCalledBySquads);

    Ok(())
}
