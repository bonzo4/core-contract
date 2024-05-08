use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::state::User;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct PayUserOptions {
    user_id: u64,
    amount: u128,
}

pub fn pay_user(ctx: Context<PayUser>, options: PayUserOptions) -> Result<()> {
    
    let signer = &mut ctx.accounts.signer;
    let usdc_payer_account = &mut ctx.accounts.usdc_payer_account;
    let user = &mut ctx.accounts.user;
    let user_vault = &mut ctx.accounts.user_vault;
    let usdc_mint = &mut ctx.accounts.usdc_mint;

    let cpi_accounts = TransferChecked {
        from: usdc_payer_account.to_account_info(),
        to: user_vault.to_account_info(),
        mint: usdc_mint.to_account_info(),
        authority: signer.to_account_info(),
    };

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            cpi_accounts,
        ),
        options.amount as u64,
        usdc_mint.decimals
    )?;
    
    user.increment_balance(options.amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(options: PayUserOptions)]
pub struct PayUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        token::mint=usdc_mint,
        token::authority=signer
    )]
    pub usdc_payer_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), options.user_id.to_string().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,
    #[account(
        mut,
        seeds = [b"user_vault".as_ref(), user.key().as_ref()],
        bump,
        token::mint=usdc_mint,
        token::authority=user
    )]
    pub user_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint=usdc_mint.key().to_string() == "BWikzMjEiqPASTPQ9Lz8aEtmsUnyR7bjD3WrmnZJgUYc"
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}