use anchor_lang::prelude::*;

use crate::{error::CoreContractErrors, state::{Team, TeamMember}};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct AddMemberOptions {
    user_id: String,
    team_id: u64,
    intial_pay: u128,
}

pub fn add_member(ctx: Context<AddMember>, options: AddMemberOptions) -> Result<()> {
    
    let signer = &mut ctx.accounts.signer;
    let team = &mut ctx.accounts.team;

    require!(team.is_owner(signer.key()), CoreContractErrors::NotAuthorized);

    let team_member = &mut ctx.accounts.team_member;
    let bump = ctx.bumps.team_member;

    

    team_member.bump = bump;
    team_member.pay = options.intial_pay;

    emit!(AddMemberEvent { user_id: options.user_id, team_id: options.team_id });
    
    Ok(())
}

#[event]
pub struct AddMemberEvent {
    user_id: String,
    team_id: u64
}

#[derive(Accounts)]
#[instruction(options: AddMemberOptions)]
pub struct AddMember<'info> {
    #[account(
        mut,
        constraint = signer.key() == team.owner,
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"team".as_ref(), options.team_id.to_string().as_ref()],
        bump,
    )]
    pub team: Account<'info, Team>,
    #[account(
        init,
        payer = signer,
        space = Team::SIZE,
        seeds = [
            b"team_member".as_ref(), 
            options.team_id.to_string().as_ref(),
            options.user_id.to_string().as_ref(),
            ],
        bump,
    )]
    pub team_member: Account<'info, TeamMember>,
    pub system_program: Program<'info, System>,
}