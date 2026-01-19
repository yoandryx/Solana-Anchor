// instructions/exchange.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::contexts;
use crate::errors::LuxError;

pub fn handler(ctx: Context<contexts::Exchange>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    require!(!escrow.is_completed, LuxError::Unauthorized);

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
