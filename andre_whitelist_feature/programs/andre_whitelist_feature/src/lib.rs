pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2gU43isSD7iPtDfWG6RiHiuVfEJ85r8z6WUXeH1THHC5");

#[program]
pub mod andre_whitelist_feature {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
