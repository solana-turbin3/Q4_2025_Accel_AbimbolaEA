use anchor_lang::prelude::*;

#[error_code]
pub enum WhitelistError {
    #[msg("The user is already whitelisted")]
    AlreadyWhitelisted,
    #[msg("The user is not whitelisted")]
    UserNotWhitelisted,
    #[msg("Failed to initialize ExtraAccountMeta list")]
    ExtraAccountMetaInitFailed,
}