use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer},
    msg,
    pubkey::log,
    sysvars::{rent::Rent, Sysvar}, ProgramResult,
};

use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;

use crate::state::Escrow;

pub fn process_take_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    msg!("Processing the Take Instruction");

    let [
        taker,
        maker, 
        mint_a,
        mint_b,
        escrow_account,
        maker_ata,
        taker_ata,
        escrow_ata,
        system_program,
        token_program,
        _associated_token_program,
        _rent_sysvar
    ] = accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };
    let taker_ata_state = pinocchio_token_2022::state::TokenAccount::from_account_info(&taker_ata)?;
    if taker_ata_state.owner() != taker.key() {
        return Err(pinocchio::program_error::ProgramError::IncorrectAuthority);
    }
    
    let bump = data[1];
    let seed = [b"escrow".as_ref(), taker.key().as_slice(), &[bump]];
    let seeds = &seed[..];

    let escrow_account_pda = derive_address(&seed, None, &crate::ID);
    log(&escrow_account_pda);
    log(&escrow_account.key());
    assert_eq!(escrow_account_pda, *escrow_account.key());
    
    let amount_to_receive = unsafe {*(data.as_ptr().add(1) as *const u64)} ;
    let amount_to_give = unsafe {*(data.as_ptr().add(1) as *const u64)};

    let bump = [bump.to_le()];
    let seed = [
        Seed::from(b"escrow"),
        Seed::from(maker.key()),
        Seed::from(&bump),
    ];
    let seeds = Signer::from(&seed);

    pinocchio_token_2022::instructions::TransferChecked {
        from: escrow_ata,
        mint: mint_a,
        to: taker_ata,
        authority: escrow_account,
        amount: amount_to_receive,
        decimals: 9,
        token_program: token_program
    }
    .invoke()?;

    Ok(())
}