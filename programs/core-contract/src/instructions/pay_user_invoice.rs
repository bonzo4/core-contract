use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::{error::CoreContractErrors, state::{User, UserInvoice}};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct PayUserInvoiceOptions {
    user_id: String,
    invoice_id: u64,
}


pub fn pay_user_invoice(ctx: Context<PayUserInvoice>, options: PayUserInvoiceOptions) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    require!(!invoice.is_paid, CoreContractErrors::InvoicePaid);
    
    let signer = &mut ctx.accounts.signer;
    let usdc_payer_account = &mut ctx.accounts.usdc_payer_account;
    let user = &mut ctx.accounts.user;
    let user_vault = &mut ctx.accounts.user_vault;
    let usdc_mint = &mut ctx.accounts.usdc_mint;
    

    let cpi_accounts = TransferChecked {
        from: usdc_payer_account.to_account_info(),
        to: user_vault.to_account_info(),
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
    user.balance += invoice.requested_amount;


    emit!(UserInvoicePaid {
        invoice_id: options.invoice_id
    });

    Ok(())
}

#[event]
pub struct UserInvoicePaid {
    pub invoice_id: u64,
}

#[derive(Accounts)]
#[instruction(options: PayUserInvoiceOptions)]
pub struct PayUserInvoice<'info> {
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
        seeds = [b"user".as_ref(), options.user_id.as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,
    #[account(
        mut,
        seeds = [b"user_vault".as_ref(), user.key().as_ref()],
        bump,
        token::mint=usdc_mint,
        token::authority=user
    )]
    pub user_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            b"user_invoice".as_ref(), 
            options.user_id.as_ref(), 
            options.invoice_id.to_string().as_ref()
            ],
        bump,
        constraint = invoice.user_id == options.user_id,
    )]
    pub invoice: Account<'info, UserInvoice>,
    #[account(
        mut,
        constraint=usdc_mint.key().to_string() == "BWikzMjEiqPASTPQ9Lz8aEtmsUnyR7bjD3WrmnZJgUYc"
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}