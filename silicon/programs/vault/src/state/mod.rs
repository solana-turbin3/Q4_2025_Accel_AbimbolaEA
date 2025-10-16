use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    bump: u8,
    admin: Pubkey,   
}