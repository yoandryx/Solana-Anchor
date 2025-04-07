use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod states;

declare_id!("GRE7cbpBscopx6ygmCvhPqMNEUDWtu9gBVSzNMSPWkLX");

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

    // Updated initialize function now accepts an extra parameter: sale_price.
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
        file_cid: String,
        luxhub_wallet: Pubkey,
        sale_price: u64
    ) -> Result<()> {
        // Call the initialize_escrow method to set up the escrow account.
        ctx.accounts.initialize_escrow(
            seed,
            &ctx.bumps,
            initializer_amount,
            taker_amount,
            file_cid,
            luxhub_wallet,
            sale_price
        )?;
        // Removed the deposit() call since no fund transfer should occur during listing.
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
