use anchor_lang::prelude::*;

use super::{DISCRIMINATOR_SIZE, PUB_KEY_SIZE, U128_SIZE, U8_SIZE};

#[account]
pub struct User {
    pub bump: u8,
    pub authority: Pubkey,
    pub balance: u128,
}

impl User {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
    + U8_SIZE
    + PUB_KEY_SIZE
    + U128_SIZE;
    

    pub fn has_authority(&self, signer: Pubkey) -> bool {
        self.authority == signer
    }


    pub fn increment_balance(&mut self, amount: u128) {
        self.balance += amount;
    }

    pub fn decrement_balance(&mut self, amount: u128) {
        self.balance -= amount;
    }
}