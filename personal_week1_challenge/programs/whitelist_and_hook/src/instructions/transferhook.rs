use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::Whitelist;


#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = user
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,

    /// The wallet/authority of the source token account (not a signer in transfer-hook callbacks)
    pub user: UncheckedAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: PDA of the vault program
    // #[account()]
    pub owner: UncheckedAccount<'info>,

    /// CHECK: ExtraAccountMetaList Account
    pub extra_account_meta_list: UncheckedAccount<'info>,

    #[account(seeds = [b"whitelist"], bump)]
    pub whitelist: Account<'info, Whitelist>,
}


impl<'info> TransferHook<'info> {
    pub fn transfer_hook(&mut self) -> Result<()> {
        // these are the instructions that must be met for the transaction to happen
        // We enforce that the token owner is present in the Whitelist.list

        // Determine the owner pubkey to check. The transfer-hook instruction gives us
        // the source token account and its authority; we'll use `source_token`'s owner
        // (authority) as the user who must be whitelisted.

        // The authority (wallet) that is performing the transfer should be provided
        // as the `user` account. Use that pubkey to check membership in the whitelist.
        // The token program calls the hook; the authority may not be a signer here. Read
        // the owner directly from the source token account data.
        let user_pk: Pubkey = self.source_token.owner;

        if !self.whitelist.list.contains(&user_pk) {
            return err!(crate::error::WhitelistError::UserNotWhitelisted);
        }

        Ok(())
    }
}
