use anchor_lang::prelude::*;

use crate::{error::CoreContractErrors, state::{Team, TeamMember}};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct EditMemberOptions {
    user_id: u64,
    team_id: u64,
    pay: u128
}

pub fn edit_member(ctx: Context<EditMember>, options: EditMemberOptions) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let team = &mut ctx.accounts.team;

    require!(team.is_owner(signer.key()), CoreContractErrors::NotAuthorized);
    let team_member = &mut ctx.accounts.team_member;

    team_member.pay = options.pay;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(options: EditMemberOptions)]
pub struct EditMember<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"team".as_ref(), options.team_id.to_string().as_ref()],
        bump,
    )]
    pub team: Account<'info, Team>,
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
    pub system_program: Program<'info, System>,
}