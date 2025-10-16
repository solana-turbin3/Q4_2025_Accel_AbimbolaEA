use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelist {
    // maximum number of pubkeys allowed in the whitelist (adjust as needed)
    #[max_len(64)]
    pub list: Vec<Pubkey>,
    pub bump: u8,
}