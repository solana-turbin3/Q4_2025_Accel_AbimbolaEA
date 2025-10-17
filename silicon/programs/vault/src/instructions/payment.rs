use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{transfer_checked_with_fee, Mint, TokenAccount, TokenInterface, TransferCheckedWithFee}
};

use crate::Vault;

#[derive(Accounts)]
pub struct Payment <'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    pub admin: Signer<'info>,
    #[account(
        mut,
        mint::decimals = 9,
        mint::authority = user,
        // extensions::transfer_hook::program_id = hook_program_id.key()
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = associated_token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, Vault>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> Payment <'info> {
    pub fn deposit(
        &mut self,
        amount: u64,
    ) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferCheckedWithFee {
            token_program_id: cpi_program.to_account_info(),
            source: self.user.to_account_info(),
            mint: self.mint.to_account_info(),
            destination: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let fees = amount * 1/100;
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked_with_fee(cpi_ctx, amount, 9, fees)?;

        Ok(())
    }

    pub fn withdraw(
        &mut self,
        amount: u64,
    ) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferCheckedWithFee {
            token_program_id: cpi_program.to_account_info(),
            source: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.vault.to_account_info(),
        };

        let fees = amount * 1/100;
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked_with_fee(cpi_ctx, amount, 9, fees)?;

        Ok(())
    }
}