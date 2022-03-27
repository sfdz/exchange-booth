use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;

#[account]
pub struct ExchangeBooth {
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub admin: Pubkey,
    // Mints are stored within the token accounts at vault0 and vault1
}

impl ExchangeBooth {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (3 * PUBLIC_KEY_LENGTH);
}

// This is a dummy oracle using a fixed-point representation for price.
// The true price is `price * 10^(exponent)`.
#[account]
pub struct Oracle {
    pub price: i64,
    pub exponent: i32
}

impl Oracle {
    pub const LEN: usize = 8 + 4;
}
