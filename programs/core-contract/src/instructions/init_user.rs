use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::User;

pub fn init_user(ctx: Context<InitUser>, user_id: u64) -> Result<()> {

    let signer = &ctx.accounts.signer;
    let user = &mut ctx.accounts.user;
    let bump = ctx.bumps.user;

    user.bump = bump;
    user.authority = *signer.key;
    user.balance = 0;

    emit!(UserCreated { user_id });

    Ok(())
}

#[event]
pub struct UserCreated {
    pub user_id: u64,
}


#[derive(Accounts)]
#[instruction(user_id: u64)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = User::SIZE,
        seeds = [b"user".as_ref(), user_id.to_string().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,
    #[account(
        init,
        payer = signer,
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