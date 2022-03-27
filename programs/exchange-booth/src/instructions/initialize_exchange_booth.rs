use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::state::{ExchangeBooth, Oracle};

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
    // TODO: convert init to init_if_needed and also protect against re-initialization attacks
    #[account(
        init,
        payer = admin,
        token::mint = mint0,
        token::authority = vault0,
        seeds = [b"vault", admin.key().as_ref(), mint0.key().as_ref()],
        bump
    )]
    // Box allocates the accounts to the heap, which is a workaround for errors like
    // `Stack offset of 4152 exceeded max offset of 4096 by 56 bytes, please minimize large stack variables`
    pub vault0: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = admin,
        token::mint = mint1,
        token::authority = vault1,
        seeds = [b"vault", admin.key().as_ref(), mint1.key().as_ref()],
        bump
    )]
    pub vault1: Box<Account<'info, TokenAccount>>,
    pub oracle: Account<'info, Oracle>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    // Required to create token accounts
    pub rent: Sysvar<'info, Rent>
}
