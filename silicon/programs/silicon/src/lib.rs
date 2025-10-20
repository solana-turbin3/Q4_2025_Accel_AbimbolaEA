pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use solana_gpt_oracle;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8TjYjuCkF7NCCcWCjFsFfrdysDXUsLXXAfK37unQJJE3");

#[program]
pub mod silicon {
    use super::*;

    pub fn initialize_agent(ctx: Context<InitializeAgent>) -> Result<()> {
        ctx.accounts.initialize_agent()
    }

    pub fn interact_agent(ctx: Context<InteractAgent>, text: String) -> Result<()> {
        ctx.accounts.interact_agent(text)
    }

    pub fn __client_accounts_callback_from_agent(
        ctx: Context<CallbackFromAgent>,
        response: String,
    ) -> Result<()> {
        ctx.accounts.callback_from_agent(response)
    }

    pub fn add_to_whitelist(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.add_to_whitelist(user, &ctx.bumps)
    }

    pub fn remove_from_whitelist(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.add_to_whitelist(user, &ctx.bumps)
    }

    pub fn extra_account_metas(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        let extra_account_metas = InitializeExtraAccountMetaList::exta_account_metas()?;
        msg!("Extra account Metas Length: {:?}", extra_account_metas);
        msg!(
            "Extra account Metas Length: {:?}",
            extra_account_metas.len()
        );

        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn hook(tx: Context<TransferHook>) -> Result<()> {
        ctx.accounts.hook()
    }
}
