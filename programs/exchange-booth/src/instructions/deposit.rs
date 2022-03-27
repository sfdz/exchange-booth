use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::state::ExchangeBooth;

// The accounts needed for the Deposit instruction
#[derive(Accounts)]
pub struct Deposit<'info> {
    pub exchange_booth: Account<'info, ExchangeBooth>,
    // Must be a signer because we're withdrawing from their token account
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    // Validate that the vault belongs to this program and admin
    #[account(mut, seeds = [b"vault", admin.key().as_ref(), mint.key().as_ref()], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token> // Will be invoked to transfer tokens
}