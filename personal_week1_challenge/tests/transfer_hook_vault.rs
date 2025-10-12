use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use litesvm::LiteSVM;

#[program]
mod transfer_hook_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.whitelist = Vec::new();
        Ok(())
    }

    pub fn add_to_whitelist(ctx: Context<WhitelistUser>, user: Pubkey, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.whitelist.push((user, amount));
        Ok(())
    }

    pub fn deposit(ctx: Context<Interact>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let user = ctx.accounts.user.key();

        if !vault.whitelist.iter().any(|(whitelisted_user, _)| whitelisted_user == &user) {
            return Err(ErrorCode::NotWhitelisted.into());
        }

        let cpi_accounts = token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Interact>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let user = ctx.accounts.user.key();

        if !vault.whitelist.iter().any(|(whitelisted_user, _)| whitelisted_user == &user) {
            return Err(ErrorCode::NotWhitelisted.into());
        }

        let cpi_accounts = token::Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = user, space = 8 + 1024)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WhitelistUser<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Interact<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub whitelist: Vec<(Pubkey, u64)>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("User is not whitelisted.")]
    NotWhitelisted,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use litesvm::LiteSVM;

    #[test]
    fn test_vault() {
        let mut svm = LiteSVM::new();

        // Initialize vault
        svm.execute(|ctx| {
            transfer_hook_vault::initialize_vault(ctx).unwrap();
        });

        // Add user to whitelist
        let user = Pubkey::new_unique();
        svm.execute(|ctx| {
            transfer_hook_vault::add_to_whitelist(ctx, user, 100).unwrap();
        });

        // Deposit funds
        svm.execute(|ctx| {
            transfer_hook_vault::deposit(ctx, 50).unwrap();
        });

        // Withdraw funds
        svm.execute(|ctx| {
            transfer_hook_vault::withdraw(ctx, 50).unwrap();
        });
    }
}