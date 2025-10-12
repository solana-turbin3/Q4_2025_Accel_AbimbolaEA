#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::TokenAccount, 
    token_2022_extensions::transfer_hook,
    token_2022::TransferChecked,
    token_interface::{Mint, TokenInterface}, 
};


declare_id!("4FwU7NpWZnCBHykLbxihDAjUcGACxHTt1erhnEfADvkP");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;

        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()], 
        bump,
        space = VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        extensions::transfer_hook::authority = owner
        extensions::transfer_hook::program_id = hook_program_id.key()
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // Get the amount of lamports needed to make the vault rent exempt
        let rent_exempt = Rent::get()?.minimum_balance(self.vault_ata.to_account_info().data_len());

        // Transfer the rent-exempt amount from the user to the vault
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.user.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        TransferChecked(cpi_ctx, rent_exempt)?;

        self.vault_state.vault_bump = bumps.vault_ata;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }  
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
      #[account(
        mut,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        extensions::transfer_hook::authority = owner
        extensions::transfer_hook::program_id = hook_program_id.key()
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {

        let cpi_program = self.associated_token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.user.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        TransferChecked(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.associated_token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        TransferChecked(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
  
    #[account(
        mut,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        extensions::transfer_hook::authority = owner
        extensions::transfer_hook::program_id = hook_program_id.key()
    )]
    pub mint: InterfaceAccount<'info, Mint>,

   #[account(
        mut,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.associated_token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.user.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        TransferChecked(cpi_ctx, self.vault_ata.lamports())?;

        Ok(())
    }
}
#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}