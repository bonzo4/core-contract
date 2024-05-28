use anchor_lang::prelude::*;

use super::{BOOL_SIZE, DISCRIMINATOR_SIZE, U128_SIZE, U64_SIZE, U8_SIZE};

#[account]
pub struct TeamInvoice {
    pub bump: u8,
    pub requested_amount: u128,
    pub is_paid: bool,
    pub team_id: u64
}

impl TeamInvoice {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
    + U8_SIZE
    + U128_SIZE
    + BOOL_SIZE
    + U64_SIZE;
}