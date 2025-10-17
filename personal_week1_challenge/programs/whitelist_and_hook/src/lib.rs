pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use spl_discriminator::discriminator::SplDiscriminate;
// use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_transfer_hook_interface::instruction::{
    ExecuteInstruction,
};

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("QKZCXvt3Ng1ku19WdFh3aHcN34M6BNS9Pm9DAhnfYBQ");

#[program]
pub mod whitelist_and_hook {

    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     ctx.accounts.initialize_whitelist(&ctx.bumps)
    // }

    pub fn add_to_whitelist(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.add_to_whitelist(user, &ctx.bumps)
    }

    pub fn remove_from_whitelist(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.remove_from_whitelist(user)
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>) -> Result<()> {
        ctx.accounts.transfer_hook()
    }

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
            msg!("Initializing Transfer Hook...");

            let extra_account_metas = InitializeExtraAccountMetaList::initialize_extra_account_meta_list()?;
            msg!("Extra account Metas Length: {:?}", extra_account_metas);
            msg!("Extra account Metas Length: {:?}", extra_account_metas.len());
        
        Ok(())
        }
    
}
