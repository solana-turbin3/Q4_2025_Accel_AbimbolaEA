use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey
};
// use crate::ID;
pub struct Fundraiser(*mut u8);
impl Fundraiser {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 2 + 1; // 91 bytes

    // 0x0xAbim
    // Creating fundraiser w/out validation
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_mut_data_unchecked().as_mut_ptr())}
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        {
            assert_eq!(*account_info.owner(), crate::ID);
            assert_eq!(account_info.data_len(), Self::LEN);
        }
        Ok(Self::from_account_info_unchecked(account_info))
    }

    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey)}
    }

    pub fn current_amount(&self) -> u64 {
        unsafe { *(self.0.add(71) as *const u64)}
    }

    // 0xAbim
    // We don't outline everything, cos we don't need all
    pub fn bump(&self) -> u8 {
        unsafe { *(self.0.add(90) as *const u8)}
    }
}



// 0xAbim 
// Fields: Maker + mint_to_raise + amount_to_raise + current amount
// + time_stated + duration + bump