use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;

// Represents the on-chain data associated with each individual exchange booth
#[account]
pub struct ExchangeBooth {
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub admin: Pubkey,
    // Mints are stored within the token accounts at vault0 and vault1
    pub oracle: Pubkey
}

impl ExchangeBooth {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (4 * PUBLIC_KEY_LENGTH);
}
