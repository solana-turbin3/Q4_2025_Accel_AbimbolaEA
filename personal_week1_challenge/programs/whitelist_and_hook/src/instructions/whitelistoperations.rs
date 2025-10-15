use anchor_lang::prelude::*;
use crate::Whitelist;
use crate::error::WhitelistError;

#[derive(Accounts)]
// #[instruction(user = Pubkey)]
pub struct WhitelistOperations <'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"whitelist", user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>
}

impl WhitelistOperations<'_> {
    pub fn add_to_whitelist(
        &mut self,
        user: Pubkey,
        // bumps: &WhitelistOperationsBumps,
    ) -> Result<()> {
        if self.whitelist.list.iter().any(|u| *u == user) {
            self.whitelist.list.push(user);
        } else {
            return err!(WhitelistError::AlreadyWhitelisted);
        }
        Ok(())
    }

    pub fn remove_from_whitelist(
        &mut self, user: Pubkey
    ) -> Result<()> {
        if let Some(index) = self.whitelist.list.iter().position(|u| *u == user) {
            self.whitelist.list.remove(index);
        } else {
            return err!(WhitelistError::UserNotWhitelisted)
        }
        Ok(())
    }
}