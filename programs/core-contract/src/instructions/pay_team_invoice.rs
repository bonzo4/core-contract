use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::{error::CoreContractErrors, state::{Team, TeamInvoice}};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct PayTeamInvoiceOptions {
    team_id: u64,
    invoice_id: u64,
}


pub fn pay_team_invoice(ctx: Context<PayTeamInvoice>, options: PayTeamInvoiceOptions) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    require!(!invoice.is_paid, CoreContractErrors::InvoicePaid);
    
    let signer = &mut ctx.accounts.signer;
    let usdc_payer_account = &mut ctx.accounts.usdc_payer_account;
    let team = &mut ctx.accounts.team;
    let team_vault = &mut ctx.accounts.team_vault;
    let usdc_mint = &mut ctx.accounts.usdc_mint;
    

    let cpi_accounts = TransferChecked {
        from: usdc_payer_account.to_account_info(),
        to: team_vault.to_account_info(),
        mint: usdc_mint.to_account_info(),
        authority: signer.to_account_info(),
    };

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            cpi_accounts,
        ),
        invoice.requested_amount as u64,
        usdc_mint.decimals
    )?;

    invoice.is_paid = true;
    team.balance += invoice.requested_amount;


    emit!(TeamInvoicePaid {
        invoice_id: options.invoice_id
    });

    Ok(())
}

#[event]
pub struct TeamInvoicePaid {
    pub invoice_id: u64,
}

#[derive(Accounts)]
#[instruction(options: PayTeamInvoiceOptions)]
pub struct PayTeamInvoice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        token::mint=usdc_mint,
        token::authority=signer
    )]
    pub usdc_payer_account: Account<'info, TokenAccount>,
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
            b"team_invoice".as_ref(), 
            options.team_id.to_string().as_ref(), 
            options.invoice_id.to_string().as_ref()
            ],
        bump,
        constraint = invoice.team_id == options.team_id,
    )]
    pub invoice: Account<'info, TeamInvoice>,
    #[account(
        mut,
        constraint=usdc_mint.key().to_string() == "BWikzMjEiqPASTPQ9Lz8aEtmsUnyR7bjD3WrmnZJgUYc"
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}