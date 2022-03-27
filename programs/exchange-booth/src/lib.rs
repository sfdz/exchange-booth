use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount, Transfer, transfer};
use state::ExchangeBooth;

pub mod state;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod exchange_booth {
    use super::*;

    // This function handles the InitializeExchangeBooth instruction.
    // We know that when this function begins executing, all accounts in the
    // InitializeExchangeBooth struct have been created successfully.
    // All that's left to do is to modify the ExchangeBooth struct, which will be
    // serialized back to the blockchain for us.
    pub fn initialize_exchange_booth(ctx: Context<InitializeExchangeBooth>) -> Result<()> {
        msg!("New accounts created! Initializing exchange booth...");

        let accounts = ctx.accounts;
        let exchange_booth = &mut accounts.exchange_booth;

        exchange_booth.vault0 = accounts.vault0.key();
        exchange_booth.vault1 = accounts.vault1.key();
        exchange_booth.admin = accounts.admin.key();

        msg!("Exchange booth initialized!");

        Ok(())
    }

    // Note that non-account instruction data is specified in this function header,
    // in this case the number of tokens to deposit.
    pub fn deposit(ctx: Context<Deposit>, deposit_amount: u64) -> Result<()> {
        // TODO: Validate that the given token belongs in this exchange booth
        let accounts = ctx.accounts;
        transfer(
            CpiContext::new(accounts.token_program.to_account_info(), Transfer {
                from: accounts.from.to_account_info(),
                to: accounts.vault.to_account_info(),
                authority: accounts.admin.to_account_info()
            }),
            deposit_amount
        )
    }

    pub fn withdraw(ctx: Context<Withdraw>, withdraw_amount: u64) -> Result<()> {
        // TODO: Validate that the given token belongs in this exchange booth
        let accounts = ctx.accounts;
        transfer(
            CpiContext::new(accounts.token_program.to_account_info(), Transfer {
                from: accounts.vault.to_account_info(),
                to: accounts.to.to_account_info(),
                authority: accounts.admin.to_account_info()
            }),
            withdraw_amount
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
    #[account(
        init,
        payer = admin,
        token::mint = mint0,
        token::authority = admin,
        seeds = [b"vault", admin.key().as_ref(), mint0.key().as_ref()],
        bump
    )]
    pub vault0: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = admin,
        token::mint = mint1,
        token::authority = admin,
        seeds = [b"vault", admin.key().as_ref(), mint1.key().as_ref()],
        bump
    )]
    pub vault1: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub exchange_booth: Account<'info, ExchangeBooth>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"vault", admin.key().as_ref(), mint.key().as_ref()], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>
}

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
