use pinocchio::{
    account_info::AccountInfo, msg, instruction::{Seed, Signer}, program_error::ProgramError, ProgramResult
};
use pinocchio_token::instructions::Transfer;

use crate::state::{Contributor, Fundraiser};

pub fn process_refund_instruction(
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    let [
        fundraiser,
        contributor_account,
        contributor_ata,
        vault,
        _token_program,
        _remaining @..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser)?;
    let contributor = Contributor::from_account_info(contributor_account)?;

    msg!("Processing refund");
    assert!(contributor.amount() > 0, "No amount to refund");
    let maker = fundraiser_account.maker();
    let bump = [fundraiser_account.bump()];
    let seed = [
        Seed::from(b"fundraiser"),
        Seed::from(&maker),
        Seed::from(&bump),
    ];
    let seeds = Signer::from(&seed);

    Transfer {
        from: vault,
        to: contributor_ata,
        authority: fundraiser,
        amount: contributor.amount()
    }.invoke_signed(&[seeds.clone()])?;

    Ok(())
}