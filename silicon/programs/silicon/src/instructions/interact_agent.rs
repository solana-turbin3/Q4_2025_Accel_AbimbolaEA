use anchor_lang::prelude::*;
use solana_gpt_oracle::{
    self,
    solana_gpt_oracle::{create_llm_context, interact_with_llm},
    ContextAccount, Counter, Identity, InteractWithLlm,
};

use crate::{constant::*, Agent, Whitelist, AGENT_DESC};

#[derive(Accounts)]
#[instruction(text: String)]
pub struct InteractAgent<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"agent"],
        bump,
    )]
    pub agent: Account<'info, Agent>,
    #[account(
        seeds = [b"whitelist", user.key().as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    /// Check: Also checked in oracle program too
    #[account(mut)]
    pub llm_context: AccountInfo<'info>,
    #[account(mut)]
    pub interaction: AccountInfo<'info>,
    #[account(mut)]
    pub counter: AccountInfo<'info, Counter>,
    #[account(address = agent.context)]
    pub context_account: Account<'info, ContextAccount>,
    /// Check: the oracle ID
    #[account(address = solana_gpt_oracle::ID)]
    pub oracle_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CallbackFromAgent<'info> {
    /// Check: Checked in the oracle program
    pub identity: Account<'info, Identity>,
    #[account(
        seeds = [b"whitelist", user.key().as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}
impl<'info> InteractAgent<'info> {
    pub fn interact_agent(&mut self, text: String) -> Result<()> {
        let cpi_program = self.oracle_program.to_account_info();
        let cpi_accounts = InteractWithLlm {
            payer: self.user.to_account_info(),
            interaction: self.interaction.to_account_info(),
            context_account: self.context_account.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let disc = instruction::CallbackFromAgent::DISCRIMINATOR
            .try_into()
            .expect("Discriminator must be 8 bytes");
        interact_with_llm(cpi_ctx, text, ID, disc, None)?;

        Ok(())
    }
}

impl<'info> CallbackFromAgent<'info> {
    pub fn callback_from_agent(&mut self, response: String) -> Result<()> {
        if !self.identity.to_account_info().is_signer {
            return Err(ProgramError::InvalidAccountData.into());
        }
        self.whitelist.set_inner(Whitelist {
            address: identity,
            is_whitelisted: true,
            bump: bumps.whitelist,
        });
        msg!("Response: {:?}", response);
        Ok(())
    }
}
