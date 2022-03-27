use anchor_lang::prelude::*;

use crate::state::Oracle;

// Create the dummy oracle account as a PDA of the exchange booth program
#[derive(Accounts)]
pub struct InitializeOracle<'info> {
    #[account(
        init,
        payer = admin,
        seeds = [b"oracle", admin.key().as_ref()],
        bump,
        space = Oracle::LEN
    )]
    pub oracle: Account<'info, Oracle>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}
