pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;
use spl_discriminator::discriminator::SplDiscriminate;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("QKZCXvt3Ng1ku19WdFh3aHcN34M6BNS9Pm9DAhnfYBQ");

#[program]
pub mod whitelist_and_hook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_whitelist(&ctx.bumps)
    }

    pub fn add_to_whitelist(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.add_to_whitelist(user)
    }

    pub fn remove_from_whitelist(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.remove_from_whitelist(user)
    }

    pub fn initialize_extra_account_meta_list(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        // call initializer and ignore the returned Vec for now; the important part is success/failure
        ctx.accounts.initialize_extra_account_meta_list().map(|_| ())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>) -> Result<()> {
        ctx.accounts.transfer_hook()
    }
}
