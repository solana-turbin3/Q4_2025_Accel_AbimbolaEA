#[cfg(target_os = "linux")]
use crate::instructions::EscrowInstructions;
use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, pubkey::Pubkey};
mod instructions;
mod state;
mod tests;

// Writing the Escrow in just a file. Wish me well.
// 0xdev didn't do that again +++++++++++++++++
// Got too complex... had to pull out ASAP!

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);

    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(pinocchio::program_error::ProgramError::InvalidInstructionData)?;

    match EscrowInstructions::try_from(discriminator)? {
        EscrowInstructions::Make => instructions::process_make_instruction(accounts, data)?,
        EscrowInstructions::Take => instructions::process_take_instruction(accounts, data)?,
        _ => return Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
    }

    
    Ok(())
}
