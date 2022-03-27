use anchor_lang::prelude::*;

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
