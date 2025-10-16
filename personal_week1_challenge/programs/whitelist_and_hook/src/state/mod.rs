use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelist {
    pub list: Pubkey,
    pub bump: u8,
}
