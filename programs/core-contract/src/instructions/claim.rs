use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::{error::CoreContractErrors, state::User};

pub fn claim(ctx: Context<Claim>, user_id: u64) -> Result<()> {

    let signer = &mut ctx.accounts.signer;
    let user = &mut ctx.accounts.user;

    require!(user.has_authority(signer.key()), CoreContractErrors::NotAuthorized);

    let recipient_payer_account = &mut ctx.accounts.recipient_payer_account;
    let user_vault = &mut ctx.accounts.user_vault;
    let usdc_mint = &mut ctx.accounts.usdc_mint;

    let cpi_accounts = TransferChecked {
        from: user_vault.to_account_info(),
        to: recipient_payer_account.to_account_info(),
        mint: usdc_mint.to_account_info(),
        authority: user.to_account_info(),
    };

    let bump = ctx.bumps.user;
    let seeds = vec![bump];
    let binding = user_id.to_string();
    let seeds = vec![b"user".as_ref(), binding.as_ref(), seeds.as_slice()];
    let seeds = vec![seeds.as_slice()];
    let seeds = seeds.as_slice();

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts, 
            seeds
        ),
        user.balance as u64,
        usdc_mint.decimals,
    )?;

    user.balance = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction(user_id: u64)]
pub struct Claim<'info> {
    #[account(
        mut,
        constraint = signer.key() == user.authority
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        token::mint=usdc_mint,
        token::authority=signer
    )]
    pub recipient_payer_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user_id.to_string().as_ref()],
        bump,
        constraint = user.balance > 0,
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