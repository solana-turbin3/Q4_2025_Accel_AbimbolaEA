use crate::state::Whitelist;
use anchor_lang::prelude::*;

#[derive(Accounts)]
// #[instruction]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + 1 + 1, // discriminator + bool + u8
        seeds = [b"whitelist", user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self, bumps: &InitializeWhitelistBumps) -> Result<()> {
        if self.whitelist.is_whitelisted {
            return Ok(());
        }
        self.whitelist.set_inner(Whitelist {
            is_whitelisted: false,
            bump: bumps.whitelist,
        });
        Ok(())
    }
}
