use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::{error::CoreContractErrors, state::{Team, TeamMember, User}, utils::is_program_manager};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct PayMemberOptions {
    team_id: u64,
    user_id: u64,
    payment_id: u64,
    amount: Option<u128>,
}

pub fn pay_member(ctx: Context<PayMember>, options: PayMemberOptions) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let team = &mut ctx.accounts.team;

    require!(is_program_manager(signer.key()) || team.is_owner(signer.key()), CoreContractErrors::NotAuthorized);
    
    let user = &mut ctx.accounts.user;
    let team_member = &mut ctx.accounts.team_member;
    let user_vault = &mut ctx.accounts.user_vault;
    let team_vault = &mut ctx.accounts.team_vault;
    let usdc_mint = &mut ctx.accounts.usdc_mint;

    let mut pay = team_member.pay;

    if options.amount.is_some() {
        pay = options.amount.unwrap();
    }

    require!(team.balance >= pay, CoreContractErrors::BalanceTooLow);

    let cpi_accounts = TransferChecked {
        from: team_vault.to_account_info(),
        to: user_vault.to_account_info(),
        authority: team.to_account_info(),
        mint: usdc_mint.to_account_info(),
    };
    
    let bump = ctx.bumps.team;
    let seeds = vec![bump];
    let binding = options.team_id.to_string();
    let seeds = vec![b"team".as_ref(), binding.as_ref(), seeds.as_slice()];
    let seeds = vec![seeds.as_slice()];
    let seeds = seeds.as_slice();

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            seeds
        ),
        pay as u64,
        usdc_mint.decimals
    )?;

    team.balance -= pay;
    user.balance += pay;

    emit!(PayMemberEvent {
        team_id: options.team_id,
        user_id: options.user_id,
        payment_id: options.payment_id,
        amount: pay,
    });

    Ok(())
}

#[event]
pub struct PayMemberEvent {
    pub team_id: u64,
    pub user_id: u64,
    pub payment_id: u64,
    pub amount: u128,
}


#[derive(Accounts)]
#[instruction(options: PayMemberOptions)]
pub struct PayMember<'info> {
    #[account(
        mut,
        constraint = 
            signer.key().to_string() == "2Xv68eQ72VpvC5J52deTYE8Ch8LYjJC1WKBtmSgistTS"
            || signer.key() == team.owner,
    )]
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
        seeds = [
            b"team_member".as_ref(),
            options.team_id.to_string().as_ref(),
            options.user_id.to_string().as_ref()
            ],
        bump,
    )]
    pub team_member: Account<'info, TeamMember>,
    #[account(
        mut,
        constraint=usdc_mint.key().to_string() == "BWikzMjEiqPASTPQ9Lz8aEtmsUnyR7bjD3WrmnZJgUYc"
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}