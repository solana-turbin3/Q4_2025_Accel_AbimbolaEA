use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

use instructions::*;
mod instructions;
mod state;
mod constant;

pinocchio_pubkey::declare_id!("86SsMcXUFs3PBJMoVWDLEyeYYKYS8HG4PBH3t3hkfn4L");

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {

    let(instruction_discriminant, instruction_data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstructions::try_from(*instruction_discriminant)? {
        FundraiserInstructions::Initialize => {
            process_initialize_instruction(accounts, instruction_data)?
        }
        FundraiserInstructions::Contribute => {
            process_conrtibute_instruction(accounts, instruction_data)?
        }
        FundraiserInstructions::CheckContributions => {
            process_check_contribution_instruction(accounts)?
        }
        FundraiserInstructions::Refund => {
            process_refund_instruction(accounts, instruction_data)?
        }
    }
    msg!("Lovely Implementation");
    Ok(())
}
