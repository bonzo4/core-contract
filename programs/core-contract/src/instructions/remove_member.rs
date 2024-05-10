use anchor_lang::prelude::*;

use crate::{error::CoreContractErrors, state::{Team, TeamMember}};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct RemoveMemberOptions {
    user_id: String,
    team_id: u64,
    remove_id: u64,
}


pub fn remove_member(ctx: Context<RemoveMember>, options: RemoveMemberOptions) -> Result<()> {
    
    let signer = &ctx.accounts.signer;
    let team = &mut ctx.accounts.team;
    
    require!(team.is_owner(signer.key()), CoreContractErrors::NotAuthorized);
    
    emit!(MemberRemoved {
        user_id: options.user_id,
        team_id: options.team_id,
        remove_id: options.remove_id,
    });
    
    Ok(())
}

#[event]
pub struct MemberRemoved {
    pub user_id: String,
    pub team_id: u64,
    pub remove_id: u64,
}


#[derive(Accounts)]
#[instruction(options: RemoveMemberOptions)]
pub struct RemoveMember<'info> {
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