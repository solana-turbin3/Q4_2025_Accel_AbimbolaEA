use anchor_lang::prelude::*;

declare_id!("HR2B3kYmiNpm4tjHpW58NNrwea2Hdkcc3BSwSrnRzzVe");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
