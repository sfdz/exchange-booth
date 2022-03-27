use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;

#[account]
pub struct ExchangeBooth {
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub admin: Pubkey,
}

impl ExchangeBooth {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (3 * PUBLIC_KEY_LENGTH);
}
