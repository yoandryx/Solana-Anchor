use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod states;

declare_id!("8667Wxa5U1LEviLgx8WedQPVH4PfnCfYGYYvHmeuZBUB");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
        file_cid: String,
    ) -> Result<()> {
        ctx.accounts
            .initialize_escrow(seed, &ctx.bumps, initializer_amount, taker_amount, file_cid)?;
        ctx.accounts.deposit(initializer_amount)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}
