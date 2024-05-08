use anchor_lang::prelude::*;

use crate::{error::CoreContractErrors, state::{TeamMember, User}};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct LeaveTeamOptions {
    user_id: u64,
    team_id: u64,
}


pub fn leave_team(ctx: Context<LeaveTeam>, _options: LeaveTeamOptions) -> Result<()> {
    
    let signer = &ctx.accounts.signer;
    let user = &mut ctx.accounts.user;
    
    require!(user.has_authority(signer.key()), CoreContractErrors::NotAuthorized);
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(options: LeaveTeamOptions)]
pub struct LeaveTeam<'info> {
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
        close = signer,
        seeds = [
            b"team_member".as_ref(),
            options.team_id.to_string().as_ref(),
            options.user_id.to_string().as_ref()
            ],
        bump,
    )]
    pub team_member: Account<'info, TeamMember>,
    pub system_program: Program<'info, System>,
}