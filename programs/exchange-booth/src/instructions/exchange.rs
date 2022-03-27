use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::state::ExchangeBooth;

// The accounts needed for the exchange instruction
#[derive(Accounts)]
pub struct Exchange<'info> {
    pub exchange_booth: Account<'info, ExchangeBooth>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: Only needed to construct PDAs which will be checked against the ExchangeBooth
    pub admin: AccountInfo<'info>,
    pub mint0: Account<'info, Mint>,
    pub mint1: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"vault", admin.key().as_ref(), mint0.key().as_ref()],
        bump
    )]
    pub vault0: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"vault", admin.key().as_ref(), mint1.key().as_ref()],
        bump
    )]
    pub vault1: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub from: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}
