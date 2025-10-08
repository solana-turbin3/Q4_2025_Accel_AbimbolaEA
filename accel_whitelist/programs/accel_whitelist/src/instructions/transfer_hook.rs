use crate::error::*;
use crate::state::Whitelist;
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{Mint, TokenAccount},
};
use std::cell::RefMut;

#[derive(Accounts)]
pub struct TransferHooks<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: owner can be anyone actually
    pub owner: UncheckedAccount<'info>,

    /// CHECK: ExtraAccountMetaList Account also will be checked by hook
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    /// CHECK: Resolved extras (whitelist PDA) passed here by Token program
    pub whitelist: UncheckedAccount<'info>, // Loaded in impl; first remaining
}

impl<'info> TransferHooks<'info> {
    /// at the point of transfer, this code executes
    pub fn transfer_hook(&mut self, _amount: u64) -> Result<()> {
        // this is actually very important. For security issues
        self.check_is_transferring()?;

        // Load resolved whitelist (auto-passed as remaining_accounts[0])
        let whitelist_info = self.whitelist.to_account_info();
        let mut data_slice: &[u8] = &whitelist_info.data.borrow();
        let whitelist = Whitelist::try_deserialize_unchecked(&mut data_slice)?;

        require!(whitelist.is_whitelisted, WhitelistError::NotWhitelisted);

        Ok(())
    }

    // the check if transferring function
    fn check_is_transferring(&mut self) -> Result<()> {
        // to confirm if the source account is token 2022
        let source_token_info = self.source_token.to_account_info();
        let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;

        // borrow above, and unpack the data here
        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;

        // search the extensions in the 165+ bytes...
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

        // what if it is in the middle of an operation?
        if !bool::from(account_extension.transferring) {
            panic!("Not Transferring");
        }

        Ok(())
    }
}
