use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;
const PRICE_LENGTH: usize = 8;
const EXPONENT_LENGTH: usize = 4;

// This is a dummy oracle using a fixed-point representation for price.
// The true price is `price * 10^(exponent)`.
#[account]
pub struct Oracle {
    pub price: i64,
    pub exponent: i32
}

impl Oracle {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + PRICE_LENGTH + EXPONENT_LENGTH;
}
