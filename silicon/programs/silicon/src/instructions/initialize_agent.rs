use anchor_lang::prelude::*;
use solana_gpt_oracle::{self, solana_gpt_oracle::create_llm_context, Counter};

use crate::{constant::*, Agent, Whitelist, AGENT_DESC};

#[derive(Accounts)]
pub struct InitializeAgent <'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 32,
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
    pub counter: AccountInfo<'info, Counter>,
    /// Check: the oracle ID
    #[account(address = solana_gpt_oracle::ID)]
    pub oracle_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeAgent<'info> {
    pub fn initialize_agent (
        &mut self
    ) -> Result<()> {
        self.agent.context = self.llm_context.key();

        // Context for the agentic tool
        let cpi_program = self.oracle_program.to_account_info();
        let cpi_accounts = solana_gpt_oracle::CreateLlmContext {
            payer: self.user.to_account_info(),
            context_account: self.llm_context.to_account_info(),
            counter: self.counter.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        create_llm_context(cpi_ctx, AGENT_DESC.to_string())?;

        Ok(())
    }
}