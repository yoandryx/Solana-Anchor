// instructions/exchange.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::Exchange;

pub fn handler(ctx: Context<Exchange>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    // is_completed and buyer==default checks are enforced in account constraints

    escrow.buyer = ctx.accounts.taker.key();
    let amount = escrow.sale_price;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.taker_funds_ata.to_account_info(),
                to: ctx.accounts.wsol_vault.to_account_info(),
                authority: ctx.accounts.taker.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}
