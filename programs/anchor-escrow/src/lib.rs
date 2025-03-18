use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod states;

declare_id!("CnKvnk5YmxWoLhb9zbf7at1vCCEH2eNhkGwfXecJEK21");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn init_admin_list(ctx: Context<InitAdminList>) -> Result<()> {
        contexts::init_admin::init_admin_list(ctx)
    }    

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
        file_cid: String,
    ) -> Result<()> {
        ctx.accounts.initialize_escrow(seed, &ctx.bumps, initializer_amount, taker_amount, file_cid)?;
        ctx.accounts.deposit(initializer_amount)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }

    pub fn add_admin(ctx: Context<AddAdmin>) -> Result<()> {
        // Note: we call the handler defined in contexts/add_list.rs
        contexts::add_list::add_admin(ctx)
    }

    pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
        // Call the mint_nft handler.
        contexts::mint_nft::mint_nft(ctx)
    }
}
