use anchor_lang::prelude::*;
use crate::states::escrow_config::EscrowConfig;
use crate::states::admin_list::AdminList;

#[derive(Accounts)]
pub struct InitializeEscrowConfig<'info> {
    #[account(
        init,
        payer = admin,
        space = EscrowConfig::SIZE,
        seeds = [b"escrow_config"],
        bump
    )]
    pub escrow_config: Account<'info, EscrowConfig>,
    #[account(mut, signer)]
    /// CHECK: This account is only used for signing and no data is read from it.
    pub admin: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_escrow_config(ctx: Context<InitializeEscrowConfig>, luxhub_wallet: Pubkey) -> Result<()> {
    let config: &mut Account<'_, EscrowConfig> = &mut ctx.accounts.escrow_config;
    config.luxhub_wallet = luxhub_wallet;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateEscrowConfig<'info> {
    #[account(mut, seeds = [b"escrow_config"], bump)]
    pub escrow_config: Account<'info, EscrowConfig>,
    // Require that the signer is an admin.
    #[account(mut, signer)]
    /// CHECK: This account is only used for signing and is trusted.
    pub admin: AccountInfo<'info>,
    pub admin_list: Account<'info, AdminList>,
}

pub fn update_escrow_config(ctx: Context<UpdateEscrowConfig>, new_luxhub_wallet: Pubkey) -> Result<()> {
    // Ensure the signer is an authorized admin.
    require!(
        ctx.accounts.admin_list.admins.contains(&ctx.accounts.admin.key()),
        ErrorCode::Unauthorized
    );

    let config = &mut ctx.accounts.escrow_config;
    config.luxhub_wallet = new_luxhub_wallet;
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access.")]
    Unauthorized,
}
