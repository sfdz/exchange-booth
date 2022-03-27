use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, transfer};
use instructions::*;

pub mod instructions;
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
        exchange_booth.oracle = accounts.oracle.key();

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

        let bump_seed = *ctx.bumps.get("vault").unwrap();
        let signer_seeds = [b"vault".as_ref(), accounts.exchange_booth.admin.as_ref(), accounts.to.mint.as_ref(), &[bump_seed]];
        transfer(
            CpiContext::new_with_signer(
                accounts.token_program.to_account_info(),
                Transfer {
                    from: accounts.vault.to_account_info(),
                    to: accounts.to.to_account_info(),
                    authority: accounts.vault.to_account_info()
                },
                &[&signer_seeds]
                ),
                withdraw_amount
        )
    }

    pub fn exchange(ctx: Context<Exchange>, source_amount: u64) -> Result<()> {
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

    // Initialize the mock oracle with the given fixed-point price
    pub fn initialize_oracle(ctx: Context<InitializeOracle>, price: i64, exponent: i32) -> Result<()> {
        let oracle = & mut ctx.accounts.oracle;

        oracle.price = price;
        oracle.exponent = exponent;

        Ok(())
    }
}
