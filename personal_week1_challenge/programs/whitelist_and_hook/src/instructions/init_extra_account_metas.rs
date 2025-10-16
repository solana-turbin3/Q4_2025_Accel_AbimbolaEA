use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenInterface}};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed
};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    // #[account(
    //     mut,
    //     // mint::
    // )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// impl<'info> InitializeExtraAccountMetaList <'info> {
//     pub fn initialize_extra_account_meta_list () -> Result<()> {
//         // they said index 0 - 3 are for the acct itself
//         // index 4 is for this account initializing
//         let account_metas = vec![
//             // put index 5 here

//         ]
//     }
// }

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn initialize_extra_account_meta_list(&mut self) -> Result<()> {
        match ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal {
                    bytes: "whitelist".as_bytes().to_vec(),
                },
            ],
            false,
            true,
        ) {
            Ok(_meta) => Ok(()),
            Err(_) => Err(anchor_lang::error::Error::from(anchor_lang::prelude::ProgramError::Custom(0))),
        }
    }
}