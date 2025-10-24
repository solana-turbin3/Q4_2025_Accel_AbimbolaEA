use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::find_program_address, ProgramResult

};
use pinocchio_token::{self, instructions::Transfer, ID};

use crate::state::Fundraiser;

pub fn process_check_contribution_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [
        user,
        user_ata,
        fundraiser,
        vault,
        _token_program,
        _remaining @..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // 0xAbim
    // Check ProgramID
    unsafe { 
        assert_eq!(*vault.owner(), ID, "This is not the owner of the vault");
    }

    let fundraiser_account = Fundraiser::from_account_info_unchecked(fundraiser);

    let seeds = [(b"fundraiser"), user.key().as_slice(), &[fundraiser_account.bump()]];
    let fundraiser_pda = find_program_address(&seeds, &crate::ID).0;
    assert_eq!(*fundraiser.key(), fundraiser_pda, "Invalid fundraiser PDA");

    let bump = [fundraiser_account.bump()];
    let seed = [
        Seed::from(b"fundraiser"),
        Seed::from(user.key()),
        Seed::from(&bump)
    ];
    let seeds = Signer::from(&seed);

    Transfer {
        from: vault,
        to: user_ata,
        authority: fundraiser,
        amount: fundraiser_account.current_amount()
    }.invoke_signed(&[seeds.clone()])?;

    Ok(())
}