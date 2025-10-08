use crate::error::*;
use crate::state::Whitelist;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
// #[instruction(init_if_needed)]
pub struct TokenFactory<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// The mint to receive (assumes already created with hook + mint auth)
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// User's ATA for the mint
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Per-user whitelist PDA
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 1 + 1, // discriminator + bool + u8
        seeds = [b"whitelist", user.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> TokenFactory<'info> {
    pub fn claim_and_whitelist(
        &mut self,
        bumps: &TokenFactoryBumps,
        amount: u64,
        // criteria_proof: [u8; 32],  // TODO: Add for tx verification
    ) -> Result<()> {
        // Mock criteria - replace with require!(verify_proof(criteria_proof), ErrorCode::InvalidProof);
        require!(
            !self.whitelist.is_whitelisted,
            WhitelistError::AlreadyWhitelisted
        );

        // CPI mint tokens
        let cpi_accounts = anchor_spl::token_interface::MintTo {
            mint: self.mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.user.to_account_info(), // User is mint authority
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        anchor_spl::token_interface::mint_to(cpi_ctx, amount)?;

        // Set whitelist
        let bump = bumps.whitelist;
        self.whitelist.set_inner(Whitelist {
            is_whitelisted: true,
            bump,
        });

        Ok(())
    }
}
