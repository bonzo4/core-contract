use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::state::User;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct PayMemberOptions {
    team_id: u64,
    user_id: u64,
    amount: u128,
}

pub fn pay_member(ctx: Context<PayMember>, options: PayMemberOptions) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(options: PayMemberOptions)]
pub struct PayMember<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
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