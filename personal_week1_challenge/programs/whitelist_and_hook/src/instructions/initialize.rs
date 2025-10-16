// use anchor_lang::prelude::*;
// // use anchor_spl::token_2022_extensions::transfer_hook;

// use crate::Whitelist;

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,

//     #[account(
//         init,
//         payer = user,
//         space = 8 + Whitelist::INIT_SPACE,
//         seeds = [b"whitelist", user.key().as_ref()],
//         bump,
//     )]
//     pub whitelist: Account<'info, Whitelist>,

//     pub system_program: Program<'info, System>,
// }

// impl<'info> Initialize<'info> {
//     pub fn initialize_whitelist(&mut self, bumps: &InitializeBumps) -> Result<()> {
//         self.whitelist.set_inner(Whitelist {
//             list:,
//             bump: bumps.whitelist,
//         });
//         Ok(())
//     }
// }
