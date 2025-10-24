// 0xAbim
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, sysvars::clock::Clock, ProgramResult
};

pub fn process_initialize_instruction(
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    let [
        fundraiser, 
        clock, 
        _remaining @ ..
        ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    unsafe {
        let clock = &*(clock.borrow_data_unchecked().as_ptr() as *const Clock);

        // 0xAbim
        // Initializing the Fundraiser Struct 
        // --------
        // bytes 0 - 31: maker
        // bytes 32 - 63: mint_to_raise
        // bytes 64 - 71: amount_to_raise
        // bytes 72 - 79: current_amount
        // bytes 80 - 87: time_started
        // bytes 88 - 89: duration this is 2 bytes to accomodate more
        // bytes 90: the Big Bump
        // the total length is 91. consisting of 90 byte_index
        let fundraiser_account = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();
        *(fundraiser_account.add(0) as *mut Pubkey) = *((data.as_ptr()).add(0) as *const Pubkey);
        *(fundraiser_account.add(32) as *mut Pubkey) = *((data.as_ptr()).add(32) as *const Pubkey);
        *(fundraiser_account.add(64) as *mut u64) = *((data.as_ptr()).add(64) as *const u64);
        *(fundraiser_account.add(80) as *mut i64) = clock.unix_timestamp;
        *(fundraiser_account.add(88) as *mut u64) = *((data.as_ptr()).add(80) as *const u64);
        *(fundraiser_account.add(90) as *mut u8) = *((data.as_ptr()).add(82) as *const u8);
    }

    Ok(())
}