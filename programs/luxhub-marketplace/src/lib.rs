use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod utils;
pub mod state;
pub mod contexts;
pub mod instructions;

// Re-export full context modules at crate root for #[program] macro
pub use contexts::initialize_config;
pub use contexts::initialize;
pub use contexts::exchange;
pub use contexts::confirm_delivery;
pub use contexts::admin_only_example;
pub use contexts::update_price;

// Also re-export the context structs directly for easier use
pub use contexts::initialize_config::InitializeConfig;
pub use contexts::initialize::Initialize;
pub use contexts::exchange::Exchange;
pub use contexts::confirm_delivery::ConfirmDelivery;
pub use contexts::admin_only_example::AdminOnlyExample;
pub use contexts::update_price::UpdatePrice;

declare_id!("kW2w2pHhAP8hFGRLganziunchKu6tjaXyomvF6jxNpj");

#[program]
pub mod luxhub_marketplace {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        squads_multisig: Pubkey,
        squads_authority: Pubkey,
    ) -> Result<()> {
        instructions::initialize_config::handler(ctx, squads_multisig, squads_authority)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
        file_cid: String,
        sale_price: u64,
    ) -> Result<()> {
        let bump: u8 = ctx.bumps.escrow;
        instructions::initialize::handler(
            ctx, seed, bump, initializer_amount, taker_amount, file_cid, sale_price,
        )
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        instructions::exchange::handler(ctx)
    }

    pub fn confirm_delivery(ctx: Context<ConfirmDelivery>) -> Result<()> {
        instructions::confirm_delivery::handler(ctx)
    }

    pub fn admin_only_example(ctx: Context<AdminOnlyExample>) -> Result<()> {
        instructions::admin_only_example::handler(ctx)
    }

    /// Update the sale price of an escrow listing.
    /// Only callable by the original seller before a buyer is assigned.
    pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
        instructions::update_price::handler(ctx, new_price)
    }
}
