use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;
pub mod error;


declare_id!("AhzB75bfztVnP2mnfTxdxtNrYmUy194XUxzYcJtzpWv5");

#[program]
pub mod core_contract {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>, user_id: String) -> Result<()> {
        instructions::init_user(ctx, user_id)
    }

    pub fn edit_user(ctx: Context<EditUser>, options: EditUserOptions) -> Result<()> {
        instructions::edit_user(ctx, options)
    }

    pub fn pay_user(ctx: Context<PayUser>, options: PayUserOptions) -> Result<()> {
        instructions::pay_user(ctx, options)
    }

    pub fn claim(ctx: Context<Claim>, options: ClaimOptions) -> Result<()> {
        instructions::claim(ctx, options)
    }

    pub fn init_team(ctx: Context<InitTeam>, team_id: u64) -> Result<()> {
        instructions::init_team(ctx, team_id)
    }

    pub fn add_member(ctx: Context<AddMember>, options: AddMemberOptions) -> Result<()> {
        instructions::add_member(ctx, options)
    }

    pub fn edit_member(ctx: Context<EditMember>, options: EditMemberOptions) -> Result<()> {
        instructions::edit_member(ctx, options)
    }

    pub fn pay_member(ctx: Context<PayMember>, options: PayMemberOptions) -> Result<()> {
        instructions::pay_member(ctx, options)
    }

    pub fn remove_member(ctx: Context<RemoveMember>, options: RemoveMemberOptions) -> Result<()> {
        instructions::remove_member(ctx, options)
    }

    pub fn leave_team(ctx: Context<LeaveTeam>, options: LeaveTeamOptions) -> Result<()> {
        instructions::leave_team(ctx, options)
    }

    pub fn pay_team(ctx: Context<PayTeam>, options: PayTeamOptions) -> Result<()> {
        instructions::pay_team(ctx, options)
    }
}
