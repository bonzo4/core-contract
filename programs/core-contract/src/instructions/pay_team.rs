use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::state::Team;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct PayTeamOptions {
    team_id: u64,
    amount: u128,
}

pub fn pay_team(ctx: Context<PayTeam>, options: PayTeamOptions) -> Result<()> {

    let signer = &mut ctx.accounts.signer;
    let usdc_payer_account = &mut ctx.accounts.usdc_payer_account;
    let team = &mut ctx.accounts.team;
    let team_vault = &mut ctx.accounts.team_vault;
    let usdc_mint = &mut ctx.accounts.usdc_mint;

    let cpi_accounts = TransferChecked {
        from: usdc_payer_account.to_account_info(),
        to: team_vault.to_account_info(),
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
    
    team.increment_balance(options.amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(options: PayTeamOptions)]
pub struct PayTeam<'info> {
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
        seeds = [b"team".as_ref(), options.team_id.to_string().as_ref()],
        bump,
    )]
    pub team: Account<'info, Team>,
    #[account(
        mut,
        seeds = [b"team_vault".as_ref(), team.key().as_ref()],
        bump,
        token::mint=usdc_mint,
        token::authority=team
    )]
    pub team_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint=usdc_mint.key().to_string() == "BWikzMjEiqPASTPQ9Lz8aEtmsUnyR7bjD3WrmnZJgUYc"
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
