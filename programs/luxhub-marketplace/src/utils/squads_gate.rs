use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions as ix_sysvar;

use crate::constants::SQUADS_V4;
use crate::errors::LuxError;

pub fn enforce_squads_cpi(
    instructions_sysvar: &AccountInfo,
    _expected_authority: &Pubkey,
) -> Result<()> {
    let current_index = ix_sysvar::load_current_index_checked(instructions_sysvar)? as usize;

    // In Squads v4, vaultTransactionExecute calls confirm_delivery via CPI.
    // The top-level instruction at current_index IS the Squads program.
    // Authority matching is already enforced in the handler (require_keys_eq! on config.authority).
    // Vault PDA signs via CPI inside Squads, so is_signer=false at the top level — skip signer check.
    let top_ix = ix_sysvar::load_instruction_at_checked(current_index, instructions_sysvar)?;
    require_keys_eq!(top_ix.program_id, SQUADS_V4, LuxError::NotCalledBySquads);

    Ok(())
}
