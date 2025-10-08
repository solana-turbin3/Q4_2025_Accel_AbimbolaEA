use anchor_lang::{
    prelude::*, system_program
};
use crate::state::Whitelist;

#[derive(Accounts)]
pub struct WhitelistFunction <'info>{
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"whiteList"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>
}

impl<'info> WhitelistFunction <'info> {
    pub fn add_to_whitelist(
        &mut self,
        address: Pubkey
    ) -> Result<()> {
        if !self.whitelist.address.contains(&address) {
            self.realloc_whitelist(true)?;
            self.whitelist.address.push(address);

            // if whitelist doesnt contain this address and 
            // realloc is_adding is true, then add the address using push
        }
        Ok(())
    }

    pub fn remove_from_whitelist(
        &mut self,
        address: Pubkey
    ) -> Result<()>  {
        if let Some(ppp) = self.whitelist.address.iter().position(|&x| x == address) {
            self.whitelist.address.remove(ppp);
            self.realloc_whitelist(false)?;

            // if there are addresses that the realloc condition
            // is false for, remove them from waitlist
        }
        Ok(())
    }

    pub fn realloc_whitelist(
        &mut self,
        is_adding:bool
    ) -> Result<()> {
        //get the account info, then calc rent, and then perform addition txn.
        let account_info = self.whitelist.to_account_info();

        if is_adding {
            let new_account_size = account_info.data_len() + std::mem::size_of::<Pubkey>();
            let lamports_req = (Rent::get()?).minimum_balance(new_account_size);
            let rent_diff = lamports_req - account_info.lamports();

            let cpi_program = self.system_program.to_account_info();
            let cpi_accounts = system_program::Transfer{
                from: self.admin.to_account_info(),
                to: account_info.clone(),
            };
            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
            system_program::transfer(cpi_context, rent_diff)?;

            //reallocating the account
            account_info.resize(new_account_size)?;
            msg!("Account Size Updated: {}", account_info.data_len());
        
        } else {
            let new_account_size = account_info.data_len() - std::mem::size_of::<Pubkey>();
            let lamports_req = (Rent::get()?).minimum_balance(new_account_size);
            let rent_diff = account_info.lamports() - lamports_req;

            //realloc the account
            account_info.resize(new_account_size)?;
            msg!("Account Size Downgraded: {}", account_info.data_len());

            **self.admin.to_account_info().try_borrow_mut_lamports()? += rent_diff;
            **self.whitelist.to_account_info().try_borrow_mut_lamports()? -= rent_diff;
        }

        Ok(())
    }
}