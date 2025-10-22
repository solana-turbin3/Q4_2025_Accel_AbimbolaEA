use pinocchio::{self, entrypoint, account_info::AccountInfo, ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::helpers::FundraisingInstructions;
mod state;
mod tests;
mod helpers;

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("8WZGo6WeJJayZLvuyf35v41snSCnptiK8i3TihkzHBb4");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);

    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraisingInstructions::try_from(discriminator) {
        FundraisingInstructions::Contribute => {}
        FundraisingInstructions::CheckContributions => {}
        FundraisingInstructions::Fundraise => {}
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    Ok(())
}

