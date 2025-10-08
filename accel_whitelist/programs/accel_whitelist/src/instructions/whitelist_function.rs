use crate::error::*;
use crate::state::Whitelist;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WhitelistFunction<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: user pubkey for seeds
    pub user: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"whitelist", user.key().as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,

    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistFunction<'info> {
    pub fn add_to_whitelist(
        &mut self,
        _user: Pubkey, // Not used; from seeds
    ) -> Result<()> {
        require!(
            !self.whitelist.is_whitelisted,
            WhitelistError::AlreadyWhitelisted
        );
        self.whitelist.set_inner(Whitelist {
            is_whitelisted: true,
            bump: self.whitelist.bump,
        });
        Ok(())
    }

    pub fn remove_from_whitelist(
        &mut self,
        _user: Pubkey, // Not used; from seeds
    ) -> Result<()> {
        require!(
            self.whitelist.is_whitelisted,
            WhitelistError::NotWhitelisted
        );
        self.whitelist.set_inner(Whitelist {
            is_whitelisted: false,
            bump: self.whitelist.bump,
        });
        Ok(())
    }
}
