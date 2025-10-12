pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("39GWCSZQBSPk6gouzFfpwMhkzAJ9Umr1Hvo3fRiDkua7");

#[program]
pub mod week_1_challenge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
