use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    user: Signer<'info>, // Acts as payer

    /// CHECK: The account use the same seeds that are already inputted there.
    #[account(
        init,
        payer = user,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = 8 + 100,  // Approx for ExtraAccountMetaList with 1 meta
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    // Define extra account metas for the transfer hook
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        // Single extra: dynamic whitelist PDA for the owner (index 3 in hook: source=0, mint=1, dest=2, owner=3)
        let seeds = &[
            Seed::Literal {
                bytes: b"whitelist".to_vec(),
            },
            Seed::AccountKey { index: 3 }, // Dynamic: copy owner pubkey from hook context
        ];
        let whitelist_meta = ExtraAccountMeta::new_with_seeds(
            seeds, false, // is_signer
            false, // is_writable (read-only check)
        )?;
        Ok(vec![whitelist_meta])
    }

    pub fn initialize_extra_account_meta_list(
        ctx: Context<Self>, // Added Context for bumps
    ) -> Result<()> {
        let extra_account_metas = Self::extra_account_metas()?;

        // Calc space/lamports like docs
        let account_size = ExtraAccountMetaList::size_of(extra_account_metas.len())? as u64;
        let lamports = Rent::get()?.minimum_balance(account_size as usize);

        let mint = ctx.accounts.mint.key();
        let bump = ctx.bumps.extra_account_meta_list;
        let signer_seeds: &[&[&[u8]]] = &[&[b"extra-account-metas", mint.as_ref(), &[bump]]];

        // CPI create the account
        let cpi_accounts = CreateAccount {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.extra_account_meta_list.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts)
            .with_signer(signer_seeds);
        create_account(cpi_ctx, lamports, account_size, &crate::ID)?;

        // Init with metas
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        )?;

        Ok(())
    }
}
