use anchor_lang::prelude::*;

use super::{DISCRIMINATOR_SIZE, U128_SIZE, U8_SIZE};

#[account]
pub struct TeamMember {
    pub bump: u8,
    pub pay: u128,
}

impl TeamMember {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
    + U8_SIZE
    + U128_SIZE;
}