use pinocchio:: {
    ProgramResult,
    account_info::AccountInfo,
    program_error::ProgramError,
    msg,
    pubkey::log,
    sysvars::{Sysvar, clock::Clock, rent::Rent}
};

use pinocchio_token::state::{TokenAccount, Mint};
use crate::state::Fundraiser;

pub fn process_fundraiser_instruction(
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    msg!("Initializing Fundraiser Instruction");

    let [
        maker,
        mint_to_raise,
        fundraiser,
        vault,
        system_program,
        token_program,
        _associated_token_program,
        _rent_sysvar
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let vault = pinocchio::

    Ok(())
}