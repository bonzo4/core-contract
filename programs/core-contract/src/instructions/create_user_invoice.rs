use anchor_lang::prelude::*;

use crate::{error::CoreContractErrors, state::{User, UserInvoice}};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct CreateUserInvoiceOptions {
    user_id: String,
    invoice_id: u64,
    requested_amount: u128,
}


pub fn create_user_invoice(ctx: Context<CreateUserInvoice>, options: CreateUserInvoiceOptions) -> Result<()> {

    let signer = &ctx.accounts.signer;
    let user = &mut ctx.accounts.user;

    require!(user.has_authority(signer.key()), CoreContractErrors::NotAuthorized);

    let invoice = &mut ctx.accounts.invoice;

    invoice.requested_amount = options.requested_amount;
    invoice.is_paid = false;
    invoice.user_id = options.user_id;


    emit!(UserInvoiceCreated {
        invoice_id: options.invoice_id,
    });

    Ok(())
}

#[event]
pub struct UserInvoiceCreated {
    pub invoice_id: u64,
}

#[derive(Accounts)]
#[instruction(options: CreateUserInvoiceOptions)]
pub struct CreateUserInvoice<'info> {
    #[account(
        mut,
        constraint = signer.key() == user.authority
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), options.user_id.as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,
    #[account(
        init,
        payer = signer,
        space = UserInvoice::SIZE,
        seeds = [
            b"user_invoice".as_ref(),  
            options.user_id.as_ref(),
            options.invoice_id.to_string().as_ref()
            ],
        bump,
    )]
    pub invoice: Account<'info, UserInvoice>,
    pub system_program: Program<'info, System>,
}
