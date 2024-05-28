use anchor_lang::prelude::*;

use crate::{error::CoreContractErrors, state::{Team, TeamInvoice}};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct CreateTeamInvoiceOptions {
    team_id: u64,
    invoice_id: u64,
    requested_amount: u128,
}


pub fn create_team_invoice(ctx: Context<CreateTeamInvoice>, options: CreateTeamInvoiceOptions) -> Result<()> {

    let signer = &ctx.accounts.signer;
    let team = &mut ctx.accounts.team;

    require!(team.is_owner(signer.key()), CoreContractErrors::NotAuthorized);

    let invoice = &mut ctx.accounts.invoice;

    invoice.requested_amount = options.requested_amount;
    invoice.is_paid = false;
    invoice.team_id = options.team_id;


    emit!(TeamInvoiceCreated {
        invoice_id: options.invoice_id,
    });

    Ok(())
}

#[event]
pub struct TeamInvoiceCreated {
    pub invoice_id: u64,
}

#[derive(Accounts)]
#[instruction(options: CreateTeamInvoiceOptions)]
pub struct CreateTeamInvoice<'info> {
    #[account(
        mut,
        constraint = signer.key() == team.owner
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
        space = TeamInvoice::SIZE,
        seeds = [
            b"team_invoice".as_ref(),  
            options.team_id.to_string().as_ref(),
            options.invoice_id.to_string().as_ref()
            ],
        bump,
    )]
    pub invoice: Account<'info, TeamInvoice>,
    pub system_program: Program<'info, System>,
}
