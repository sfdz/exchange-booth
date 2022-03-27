use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::state::ExchangeBooth;

// The accounts needed for the InitializeExchangeBooth instruciton
#[derive(Accounts)]
pub struct InitializeExchangeBooth<'info> {
    // This macro does a lot of heavy lifing to invoke the system program and create the PDA we want.
    // The address is unique to the admin + an _ordered_ pair of the mints.
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
    // Note that the PDA of the vaults is independent of the exchange booth,
    // so the admin can deposit to one vault and the funds will provide liquidity
    // to all of their exchange booths that use that token.
    // TODO: convert init to init_if_needed and protect against re-initialization attacks
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
    // Required to create token accounts
    pub rent: Sysvar<'info, Rent>
}
