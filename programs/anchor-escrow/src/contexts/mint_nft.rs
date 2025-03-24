use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
use crate::states::admin_list::AdminList;
use crate::contexts::add_list::CustomError;

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub admin_list: Account<'info, AdminList>,
    #[account(mut, signer)]
    /// CHECK: The admin must be one of the admins in AdminList.
    pub admin: AccountInfo<'info>,
    /// CHECK: The recipient (seller) to whom the NFT will be assigned.
    pub recipient: AccountInfo<'info>,
    #[account(
        init, 
        payer = admin,
        mint::decimals = 0, 
        mint::authority = admin, 
        mint::freeze_authority = admin
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        init, 
        payer = admin,
        associated_token::mint = nft_mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
    // Ensure the admin is authorized.
    require!(
        ctx.accounts.admin_list.admins.contains(ctx.accounts.admin.key),
        CustomError::Unauthorized
    );

    // Mint 1 token (NFT) to the recipient's associated token account.
    let cpi_accounts = MintTo {
        mint: ctx.accounts.nft_mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::mint_to(CpiContext::new(cpi_program, cpi_accounts), 1)?;
    Ok(())
}
