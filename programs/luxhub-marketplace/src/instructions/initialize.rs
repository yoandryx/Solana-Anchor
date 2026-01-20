use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::{Initialize, state::Escrow, errors::LuxError};

pub fn handler(
    ctx: Context<Initialize>,
    seed: u64,                // bookkeeping (not used for PDA seeds)
    bump: u8,                 // from ctx.bumps.escrow
    initializer_amount: u64,  // expect 1 for an NFT
    taker_amount: u64,
    file_cid: String,
    sale_price: u64,
) -> Result<()> {
    // Basic sanity
    require!(initializer_amount == 1, LuxError::Unauthorized);

    // Populate escrow
    let escrow: &mut Account<Escrow> = &mut ctx.accounts.escrow;
    escrow.seed = seed;
    escrow.bump = bump;
    escrow.initializer = ctx.accounts.seller.key();
    escrow.luxhub_wallet = ctx.accounts.config.squads_multisig; // <-- fixed
    escrow.mint_a = ctx.accounts.mint_a.key();
    escrow.mint_b = ctx.accounts.mint_b.key();
    escrow.initializer_amount = initializer_amount;
    escrow.taker_amount = taker_amount;
    escrow.file_cid = file_cid;
    escrow.sale_price = sale_price;
    escrow.is_completed = false;
    escrow.buyer = Pubkey::default();

    // Move the NFT to the escrow vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.seller_ata_b.to_account_info(),
                to: ctx.accounts.nft_vault.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        ),
        initializer_amount,
    )?;

    Ok(())
}
