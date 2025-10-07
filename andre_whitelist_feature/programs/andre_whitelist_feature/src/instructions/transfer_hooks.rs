use std::cell::RefMut;
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
    extension::{
        transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut, PodStateWithExtensionsMut
    },
    pod::PodAccount
    },
    token_interface::{
        Mint, TokenAccount
    }
};
use crate::state::Whitelist;

#[derive(Accounts)]
pub struct TransferHooks <'info> {
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

    ///CHECK: owner can be anyone actually
    pub owner: UncheckedAccount<'info>,

    ///CHECK: ExtraAccountMetaList Account also will be checked by hook
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    #[account(
        seeds = [b"whiteList"],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>
}

impl<'info> TransferHooks <'info> {
    ///at the point of transfer, thsi code executes
    pub fn transfer_hook(
        &mut self,
        _amount: u64
    ) -> Result<()> {
        //this is actually very important. For security issues
        self.check_is_transferring()?;
        if !self.whitelist.address.contains(self.owner.key) {
            panic!("TransferHook: Owner is not whitelisted");
        };

        Ok(())
    }

    //the check if transferring function
    fn check_is_transferring(&mut self) -> Result<()> {
        // to confirm if the source account is token 2022 
        let source_token_info = self.source_token.to_account_info();
        let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;

        //borrow above, and unpack the data here
        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;

        // search the extensions in the 165+ bytes...
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

        //what if it is in the middle of an operation?
        if !bool::from(account_extension.transferring) {
            panic!("TransferHook: Not transferring, but why nauuu???");
        }

        Ok(())
    }
}