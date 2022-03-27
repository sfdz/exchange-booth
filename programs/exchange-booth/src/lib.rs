use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, Mint, TokenAccount, Transfer, transfer}, associated_token::AssociatedToken};
use state::ExchangeBooth;

pub mod state;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod exchange_booth {
    use super::*;

    pub fn initialize_exchange_booth(ctx: Context<InitializeExchangeBooth>, deposit_amount0: u64, deposit_amount1: u64) -> Result<()> {
        let result = transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.admin_token0.to_account_info(),
                to: ctx.accounts.vault0.to_account_info(),
                authority: ctx.accounts.admin.to_account_info()
            }),
            deposit_amount0
        );

        match result {
            Err(e) => return Err(e),
            Ok(_) => ()
        }

        // We know that the first transfer succeeded, so the overall result depends on the next transfer
        transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.admin_token1.to_account_info(),
                to: ctx.accounts.vault1.to_account_info(),
                authority: ctx.accounts.admin.to_account_info()
            }),
            deposit_amount1
        )
    }
}

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
    #[account(mut)]
    pub admin_token0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub admin_token1: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = admin,
        token::mint = mint0,
        token::authority = admin,
        seeds = [b"vault", exchange_booth.key().as_ref(), b"0"],
        bump
    )]
    pub vault0: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = admin,
        token::mint = mint1,
        token::authority = admin,
        seeds = [b"vault", exchange_booth.key().as_ref(), b"1"],
        bump
    )]
    pub vault1: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

