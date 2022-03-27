use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::state::ExchangeBooth;

#[derive(Accounts)]
pub struct InitializeExchangeBooth<'info> {
    #[account(
        init,
        payer = admin,
        seeds = [b"exchange_booth", admin.key().as_ref(), mint0.key().as_ref(), mint1.key().as_ref()],
        bump,
        space = ExchangeBooth::LEN
    )]
    pub exchange_booth: Account<'info, ExchangeBooth>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint0: Account<'info, Mint>,
    pub mint1: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        token::mint = mint0,
        token::authority = vault0,
        seeds = [b"vault", admin.key().as_ref(), mint0.key().as_ref()],
        bump
    )]
    pub vault0: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = admin,
        token::mint = mint1,
        token::authority = vault1,
        seeds = [b"vault", admin.key().as_ref(), mint1.key().as_ref()],
        bump
    )]
    pub vault1: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}
