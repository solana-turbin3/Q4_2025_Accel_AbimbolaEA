use anchor_lang::prelude::*;

#[error_code]
pub enum WhitelistError {
    #[msg("Owner is not whitelisted")]
    NotWhitelisted,
    #[msg("Not in the middle of a transfer")]
    NotTransferring,
    #[msg("Invalid criteria proof")]
    InvalidProof,
    #[msg("Already whitelisted")]
    AlreadyWhitelisted,
    #[msg("Bump error")]
    BumpError,
}
