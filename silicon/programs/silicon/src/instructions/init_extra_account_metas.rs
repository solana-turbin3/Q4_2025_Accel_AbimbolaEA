use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed,
};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn exta_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        OK(
            vec![
                ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::Literal { bytes: b"whitelist".to_vec(), 
                    },
                    Seed::AccountKey { index: (0) },
                    ],
                    false,
                    false,
                ).unwrap()
            ]
        )
    }
}