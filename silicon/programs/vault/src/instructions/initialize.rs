use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::token_2022::{
    initialize_mint2,
    spl_token_2022::{
        extension::{
            transfer_fee::TransferFeeConfig, BaseStateWithExtensions, StateWithExtensions,
        },
        pod::PodMint,
        state::Mint as MintState,
    },
    InitializeMint2,
};
use anchor_spl::token_interface::spl_pod::optional_keys::OptionalNonZeroPubkey;
use anchor_spl::token_interface::{transfer_fee_initialize, TokenAccount};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::extension::ExtensionType,
    token_interface::{Mint, TokenInterface, TransferFeeInitialize},
};

use crate::state::Vault;
use crate::constants::SILICON_ID;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub admin: Signer<'info>,
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        extensions::transfer_hook::program_id = SILICON_ID.key()
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = associated_token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault.set_inner(Vault {
            bump: bumps.vault,
            admin: self.user.key(),
        });

        Ok(())
    }
    pub fn transfer_fee_ix(
        &mut self,
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
    ) -> Result<()> {
        let mint_size = ExtensionType::try_calculate_account_len::<PodMint>(&[
            ExtensionType::TransferFeeConfig,
        ])?;
        let lamports = (Rent::get()?).minimum_balance(mint_size);

        create_account(
            CpiContext::new(
                self.associated_token_program.to_account_info(),
                CreateAccount {
                    from: self.user.to_account_info(),
                    to: self.admin.to_account_info(),
                },
            ),
            lamports,
            mint_size as u64,
            &self.associated_token_program.key(),
        )?;

        transfer_fee_initialize(
            CpiContext::new(
                self.associated_token_program.to_account_info(),
                TransferFeeInitialize {
                    token_program_id: self.associated_token_program.to_account_info(),
                    mint: self.mint.to_account_info(),
                },
            ),
            Some(&self.admin.key()), //Config authority
            Some(&self.admin.key()), //Withdraw authority
            transfer_fee_basis_points,
            maximum_fee,
        )?;

        initialize_mint2(
            CpiContext::new(
                self.associated_token_program.to_account_info(),
                InitializeMint2 {
                    mint: self.mint.to_account_info(),
                },
            ),
            2,                //decimals
            &self.user.key(), //mint authority
            Some(&self.admin.key()),
        )?;
        self.check_mint_data()?;
        Ok(())
    }

    pub fn check_mint_data(&mut self) -> Result<()> {
        let mint = &self.mint.to_account_info();
        let mint_data = mint.data.borrow();
        let mint_with_extension = StateWithExtensions::<MintState>::unpack(&mint_data)?;
        let extension_data = mint_with_extension.get_extension::<TransferFeeConfig>()?;

        assert_eq!(
            extension_data.transfer_fee_config_authority,
            OptionalNonZeroPubkey::try_from(Some(self.user.key()))?
        );

        assert_eq!(
            extension_data.withdraw_withheld_authority,
            OptionalNonZeroPubkey::try_from(Some(self.user.key()))?
        );

        msg!("{:?}", extension_data);
        Ok(())
    }
}
