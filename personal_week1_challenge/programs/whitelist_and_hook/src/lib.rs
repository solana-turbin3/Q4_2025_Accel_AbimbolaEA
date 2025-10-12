pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("QKZCXvt3Ng1ku19WdFh3aHcN34M6BNS9Pm9DAhnfYBQ");

#[program]
pub mod whitelist_and_hook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
