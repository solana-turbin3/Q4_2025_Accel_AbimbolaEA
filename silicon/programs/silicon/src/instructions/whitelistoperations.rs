use anchor_lang::prelude::*;
use crate::{
    Whitelist,
    error::WhitelistError,
};
#[derive(Accounts)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user, 
        space = Whitelist::DISCRIMINATOR.len() + Whitelist::INIT_SPACE,
        seeds = [b"whitelist", user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations <'info> {
    pub fn add_to_whitelist (
        &mut self,
        user: Pubkey,
        bumps: &WhitelistOperationsBumps,
    ) -> Result<()> {
        if self.whitelist.address == user {
            return err!(WhitelistError::AlreadyWhitelisted);
        }
        self.whitelist.set_inner(Whitelist { address: user, bump: bumps.whitelist });
        Ok(())
    }

    pub fn remove_from_whitelist(
        &mut self,
    ) -> Result<()> {
        if self.whitelist.address != user {
            return err!(WhitelistError::UserNotWhitelisted)
        }
        self.whitelist.close(self.user.to_account_info())?;
        Ok(())
    }
}