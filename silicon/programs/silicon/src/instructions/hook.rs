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

use crate::{error::WhitelistError, Whitelist};

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: The PDA of the VaultProgram
    pub owner: UncheckedAccount<'info>,

    /// CHECK: ExtraAccountMetaListAccount
    pub extra_account_meta_list: UncheckedAccount<'info>,

    #[account(
        seeds = [b"whitelist", user.key().as_ref()],
        bump = whitelist.bump,
        // seeds::program =
    )]
    pub whitelist: Account<'info, Whitelist>,
}

impl<'info> TransferHook<'info> {
    pub fn hook(&mut self) -> Result<()> {
        self.check_is_transferring()?;

        if self.whitelist.address != self.soutce_token.key() {
            return err!(WhitelistError::UserNotWhitelisted);
        }
        Ok(())
    }

    fn check_is_transferring(mut self) -> Result<()> {
        let source_token_info = self.source_token.to_account_info();
        let mut account_data_ref = source_token_info.try_borrow_mut_data()?;

        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;
        if !bool::from(account_extension.transferring) {
            panic!("TransferHook: Not Transferring");
        }

        Ok(())
    }
}
