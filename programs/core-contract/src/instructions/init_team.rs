use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::Team;

pub fn init_team(ctx: Context<InitTeam>, team_id: u64) -> Result<()> {
   
   let signer = &ctx.accounts.signer;
   let team = &mut ctx.accounts.team;
   let bump = ctx.bumps.team;

   team.bump = bump;
   team.owner = signer.key();
   team.balance = 0;

   emit!(TeamCreated { team_id });
   
   Ok(())   
}

#[event]
pub struct TeamCreated {
    pub team_id: u64,
}


#[derive(Accounts)]
#[instruction(team_id: u64)]
pub struct InitTeam<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Team::SIZE,
        seeds = [b"team".as_ref(), team_id.to_string().as_ref()],
        bump,
    )]
    pub team: Account<'info, Team>,
    #[account(
        init,
        payer = signer,
        seeds = [b"team_vault".as_ref(), team.key().as_ref()],
        bump,
        token::mint=usdc_mint,
        token::authority=team
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