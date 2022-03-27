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

    pub fn exchange(ctx: Context<Exchange>, source_amount: u64, ) -> Result<()> {
        // TODO: Validate that the provided accounts match the mints

        let accounts = ctx.accounts;

        // TODO: Determine how much of the other token the user should get
        let destination_amount = source_amount;

        // Determine which vault is receiving user tokens and which will send tokens out.
        // We also need to get the bump seed for the vault sending tokens out. This is for the PDA signature.
        let (from_vault, to_vault, to_bump) = if accounts.from.mint == accounts.vault0.mint {
            (& mut accounts.vault0, & mut accounts.vault1, *ctx.bumps.get("vault1").unwrap())
        } else {
            (& mut accounts.vault1, & mut accounts.vault0, *ctx.bumps.get("vault0").unwrap())
        };

        let first_transfer_result = transfer(
            CpiContext::new(accounts.token_program.to_account_info(), Transfer {
                from: accounts.from.to_account_info(),
                to: from_vault.to_account_info(),
                authority: accounts.user.to_account_info()
            }),
            source_amount
        );
        match first_transfer_result {
            Err(e) => return Err(e),
            Ok(_) => ()
        }

        let signer_seeds = [b"vault".as_ref(), accounts.exchange_booth.admin.as_ref(), accounts.to.mint.as_ref(), &[to_bump]];
        transfer(
            CpiContext::new_with_signer(
                accounts.token_program.to_account_info(),
                Transfer {
                    from: to_vault.to_account_info(),
                    to: accounts.to.to_account_info(),
                    authority: to_vault.to_account_info() // The PDA itself is the authority
                },
                &[&signer_seeds]
            ),
            destination_amount
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
