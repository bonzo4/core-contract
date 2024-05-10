use anchor_lang::prelude::*;

use crate::{error::CoreContractErrors, state::User, utils::is_program_owner};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct EditUserOptions {
    user_id: String,
    edit_id: u64,
    new_authority: Pubkey,
}

pub fn edit_user(ctx: Context<EditUser>, options: EditUserOptions) -> Result<()> {

    let signer = &ctx.accounts.signer;
    require!(is_program_owner(signer.key()), CoreContractErrors::NotAuthorized);
    
    let user = &mut ctx.accounts.user;
    user.authority = options.new_authority;

    emit!(EditUserEvent {
        user_id: options.user_id,
        edit_id: options.edit_id,
        new_authority: options.new_authority,
    });
    
    Ok(())
}

#[event]
pub struct EditUserEvent {
    pub user_id: String,
    pub edit_id: u64,
    pub new_authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(options: EditUserOptions)]
pub struct EditUser<'info> {
    #[account(
        mut, 
        constraint = signer.key().to_string() == "9pT6i1LSxsFUd3jX8a3LfPV5A5UqS9mQdU3REPAM9Uev"
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), options.user_id.to_string().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}