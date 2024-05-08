use anchor_lang::prelude::*;

use super::{DISCRIMINATOR_SIZE, PUB_KEY_SIZE, U128_SIZE, U8_SIZE};

#[account]
pub struct TeamMember {
    pub bump: u8,
    pub user_id: u64,
    pub pay: u128,
}

impl TeamMember {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
    + U8_SIZE
    + PUB_KEY_SIZE
    + U128_SIZE;

    pub fn has_authority(&self, user_id: u64) -> bool {
        self.user_id == user_id
    }
}