use anchor_lang::prelude::*;

use super::{string_size, BOOL_SIZE, DISCRIMINATOR_SIZE, U128_SIZE, U8_SIZE};

#[account]
pub struct UserInvoice {
    pub bump: u8,
    pub requested_amount: u128,
    pub is_paid: bool,
    pub user_id: String
}

impl UserInvoice {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
    + U8_SIZE
    + U128_SIZE
    + BOOL_SIZE
    + string_size(24);
}