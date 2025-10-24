use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult
};
use pinocchio_token::instructions::Transfer;
use crate::constant::*;

pub fn process_conrtibute_instruction(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let amount: u64 = unsafe { *(data.as_ptr() as *const u64)};

    assert!(amount >= MINIMUM_AMT, "Amount too low");
    assert!(amount >= MAXIMUM_AMT, "Amount too high");

    let [user, contributor, user_ata, fundraiser, vault, _token_program, _ramaining @ ..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Transfer {
        from: user_ata,
        to: vault,
        authority: user,
        amount,
    }.invoke()?;

    unsafe {
        // 0xAbim
        // Updating both states to reflect amount
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(72) as *mut u64) += amount;
        *(contributor.borrow_mut_data_unchecked().as_mut_ptr().add(0) as *mut u64) += amount;
    }
    Ok(())

    // let transfer_contribution = TransferChecked {
    //     from: from,
    //     mint,
    //     to:,
    //     authority,
    //     amount,
    //     decimals
    // };

    // let seeds
}