use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;

const PROGRAM_ID: &str = "86SsMcXUFs3PBJMoVWDLEyeYYKYS8HG4PBH3t3hkfn4L";

#[test]
fn test_program() {
    // Load the compiled program
    let program_data = include_bytes!("../target/deploy/p_fundraiser.so");
    let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();

    // Create a new LiteSVM instance
    let mut svm = LiteSVM::new();

    // Add the program to the SVM
    svm.add_program(program_id, program_data);

    // Create a test keypair
    let payer = Keypair::new();

    // Airdrop some SOL to the payer
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    // Create an instruction
    let instruction = Instruction::new_with_bytes(
        program_id,
        &[], // Empty instruction data
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint_to.pubkey(), true),
        ],
    );

    // Create and send transaction
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "Transaction failed: {:?}", result.err());
}
