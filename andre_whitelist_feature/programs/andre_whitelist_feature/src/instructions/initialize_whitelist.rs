use anchor_lang::prelude::*;
use crate::state::Whitelist;

#[derive(Accounts)]
pub struct InitializeWhitelist <'info> {
    #[account(mut)]
    pub admin: Signer <'info>,

        #[account(
            init,
            payer = admin,
            space = 8 + 4 + 1, // 8 for the discrim,  4 for the vector length, and 1 for bump
            seeds = [b"whiteList"],
            bump
        )]
        pub whitelist: Account<'info, Whitelist>,

        pub system_program: Program<'info, System>
}

impl <'info> InitializeWhitelist <'info> {
    pub fn initialize_whitelist(
        &mut self,
        bumps: &InitializeWhitelistBumps
    ) -> Result<()> {
        self.whitelist.set_inner( Whitelist {
            address: vec![],
            bump: bumps.whitelist,
        });

        Ok(())
    }
}