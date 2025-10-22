use pinocchio::{account_info::{AccountInfo}, program_error::ProgramError};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fundraiser {
    maker: [u8; 32],
    mint_to_raise: [u8; 32],
    amount_to_raise: [u8; 8],
    current_amount: [u8; 8],
    time_started: i64,
    duration: i64,
    pub bump: [u8; 1],
}

impl Fundraiser {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 8 + 1;
    pub fn account_info(account_info: &AccountInfo) ->Result<&mut Self, ProgramError> {
        let mut data = account_info.try_borrow_mut_data()?;
        if data.len() != Fundraiser::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        if (data.as_ptr() as usize) & core::mem::align_of::<Self>() != 0 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self)})
    }

    pub fn maker(&self) -> pinocchio::pubkey::Pubkey {
        pinocchio::pubkey::Pubkey::from(self.maker)
    }

    pub fn mint_to_raise(&self) -> pinocchio::pubkey::Pubkey {
        pinocchio::pubkey::Pubkey::from(self.mint_to_raise)
    }

    pub fn amount_to_raise(&self) -> u64 {
        u64::from_le_bytes(self.amount_to_raise)
    }

    pub fn set_amount_to_raise(&mut self, amount: u64) {
        self.amount_to_raise = amount.to_le_bytes();
    }
    pub fn current_amount(&self) -> u64 {
        u64::from_le_bytes(self.current_amount)
    }

    pub fn time_started(&self) -> i64 {
        i64::from_le(self.time_started)
    }

    pub fn duration(&self) -> i64 {
        i64::from_le(self.duration)
    }

}