use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Mint};
use crate::contexts::add_list::CustomError;
use crate::states::admin_list::AdminList;

#[derive(Accounts)]
pub struct RestrictedTransfer<'info> {
    /// CHECK: Verified in logic that this pubkey is in AdminList
    #[account(mut)]
    pub admin_list: Account<'info, AdminList>,

    /// CHECK: This is the admin account, verified in the logic to be in AdminList.
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,

    // The mint of the NFT we are transferring
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,

    // The current owner's token account (the sender)
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,

    // The new owner's token account (the receiver)
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn restricted_transfer(ctx: Context<RestrictedTransfer>, amount: u64) -> Result<()> {
    // 1) Check if the admin is authorized
    require!(
        ctx.accounts.admin_list.admins.contains(ctx.accounts.admin.key),
        CustomError::Unauthorized
    );

    // 2) We can confirm that `nft_mint.freeze_authority == admin.key()` if we want
    //    to ensure the admin is the freeze authority. Not strictly required, but an extra check.
    // NOTE: This step is optional and only possible if you
    //       have not removed freeze_authority from the mint.
    // let mint_info = &ctx.accounts.nft_mint;
    // require!(
    //     mint_info.freeze_authority == COption::Some(ctx.accounts.admin.key()),
    //     CustomError::Unauthorized
    // );

    // 3) Perform the actual transfer (SPL Token)
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_ata.to_account_info(),
        to: ctx.accounts.to_ata.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        amount,
    )?;

    Ok(())
}
