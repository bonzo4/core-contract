use anchor_lang::prelude::*;

use super::{DISCRIMINATOR_SIZE, PUB_KEY_SIZE, U128_SIZE, U8_SIZE};

#[account]
pub struct Team {
    pub bump: u8,
    pub owner: Pubkey,
    pub balance: u128,
}

impl Team {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
    + U8_SIZE
    + PUB_KEY_SIZE
    + U128_SIZE;

    pub fn is_owner(&self, owner: Pubkey) -> bool {
        self.owner == owner
    }

    pub fn increment_balance(&mut self, amount: u128) {
        self.balance += amount;
    }

    pub fn decrement_balance(&mut self, amount: u128) {
        self.balance -= amount;
    }
}