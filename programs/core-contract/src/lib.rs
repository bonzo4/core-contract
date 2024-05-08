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

    pub fn init_user(ctx: Context<InitUser>, user_id: u64) -> Result<()> {
        instructions::init_user(ctx, user_id)
    }

    pub fn edit_user(ctx: Context<EditUser>, options: EditUserOptions) -> Result<()> {
        instructions::edit_user(ctx, options)
    }

    pub fn pay_user(ctx: Context<PayUser>, options: PayUserOptions) -> Result<()> {
        instructions::pay_user(ctx, options)
    }

    pub fn claim(ctx: Context<Claim>, user_id: u64) -> Result<()> {
        instructions::claim(ctx, user_id)
    }
}
