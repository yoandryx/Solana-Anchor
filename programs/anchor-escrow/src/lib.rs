// lib.rs
use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod states;

declare_id!("GRE7cbpBscopx6ygmCvhPqMNEUDWtu9gBVSzNMSPWkLX");

#[error_code]
pub enum ErrorCode {
    #[msg("Calculation error.")]
    CalculationError,
}

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn init_admin_list(ctx: Context<InitAdminList>) -> Result<()> {
        contexts::init_admin::init_admin_list(ctx)
    }    

    pub fn initialize_escrow_config(ctx: Context<InitializeEscrowConfig>, luxhub_wallet: Pubkey) -> Result<()> {
        contexts::escrow_config::initialize_escrow_config(ctx, luxhub_wallet)
    }

    pub fn update_escrow_config(ctx: Context<UpdateEscrowConfig>, new_luxhub_wallet: Pubkey) -> Result<()> {
        contexts::escrow_config::update_escrow_config(ctx, new_luxhub_wallet)
    }

    // The initialize instruction now extracts the escrow bump directly from the generated bumps struct.
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
        file_cid: String,
        luxhub_wallet: Pubkey,
        sale_price: u64,
    ) -> Result<()> {
        // Directly access the escrow bump from the bumps struct.
        let bump = ctx.bumps.escrow;
        ctx.accounts.initialize_escrow(
            seed,
            bump,
            initializer_amount,
            taker_amount,
            file_cid,
            luxhub_wallet,
            sale_price,
        )?;
        // No deposit() call is made here.
        Ok(())
    }
    
    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }

    pub fn add_admin(ctx: Context<AddAdmin>) -> Result<()> {
        contexts::add_list::add_admin(ctx)
    }

    pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
        contexts::mint_nft::mint_nft(ctx)
    }
    
    pub fn confirm_delivery(ctx: Context<ConfirmDelivery>) -> Result<()> {
        contexts::exchange::confirm_delivery(ctx)
    }

    pub fn restricted_transfer_instruction(
        ctx: Context<RestrictedTransfer>,
        amount: u64,
    ) -> Result<()> {
        restricted_transfer(ctx, amount)
    }
}
