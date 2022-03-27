use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;

#[account]
pub struct ExchangeBooth {
    mint0: Pubkey,
    mint1: Pubkey,
    admin: Pubkey,
    // seeds = ["token", exchange booth address, "token0"]
}

impl ExchangeBooth {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (3 * PUBLIC_KEY_LENGTH);
}
