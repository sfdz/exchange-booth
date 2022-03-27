use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::state::ExchangeBooth;

// The accounts needed for the Withdraw instruction
#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub exchange_booth: Account<'info, ExchangeBooth>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"vault", admin.key().as_ref(), mint.key().as_ref()], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>
}
