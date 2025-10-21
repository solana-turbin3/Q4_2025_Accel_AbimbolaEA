#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use litesvm::LiteSVM;
    use litesvm_token::{
        CreateAssociatedTokenAccount, CreateMint, MintTo,
        spl_token::{self},
    };
    // use pinocchio::{msg, program};
    use solana_instruction::{AccountMeta, Instruction};
    use solana_keypair::Keypair;
    use solana_message::Message;
    use solana_native_token::LAMPORTS_PER_SOL;
    use solana_program::{msg, rent::Rent, sysvar::SysvarId};
    use solana_pubkey::Pubkey;
    use solana_transaction::Transaction;
    use spl_associated_token_account::solana_program::pubkey::Pubkey as SplPubkey;
    // use pinocchio::pubkey::Pubkey;
    use solana_signer::Signer;

    const PROGRAM_ID: &str = "4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT";
    const TOKEN_PROGRAM_ID: Pubkey = spl_token::ID;
    const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

    fn program_id() -> Pubkey {
        Pubkey::from(crate::ID)
    }

    fn setup() -> (LiteSVM, Keypair) {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)
            .expect("Airdrop failed");

        // Loading the .so file to the runtime
        // 0x0x0x0x0x0x00xMem e/acc Cracked by design

        msg!("The path is!! {}", env!("CARGO_MANIFEST_DIR"));
        let so_path = PathBuf::from(
            "/home/eaa/accel_builders/repo/damos/pi-escrow/target/sbf-solana-solana/release/pi_escrow.so",
        );
        msg!("The path is {:?}", so_path);

        msg!("The program ID is: {:?}", crate::id());

        let program_data = std::fs::read(so_path).expect("Failed to read program SO file");
        svm.add_program(program_id(), &program_data);
        (svm, payer)
    }

    #[test]
    pub fn test_make_instruction() {
        let (mut svm, payer) = setup();
        let program_id = program_id();
        assert_eq!(program_id.to_string(), PROGRAM_ID);

        let mint_a = CreateMint::new(&mut svm, &payer)
            .decimals(6)
            .authority(&payer.pubkey())
            .send()
            .unwrap();
        msg!("Mint A: {}", mint_a);

        let mint_b = CreateMint::new(&mut svm, &payer)
            .decimals(6)
            .authority(&payer.pubkey())
            .send()
            .unwrap();
        msg!("Mint B: {}", mint_b);

        // 0x0x0x0x00x e/acc
        // Creating unique ATAs for the subs
        let maker_ata_a = CreateAssociatedTokenAccount::new(&mut svm, &payer, &mint_a)
            .owner(&payer.pubkey())
            .send()
            .unwrap();
        msg!("Maker ATA A: {}\n", maker_ata_a);

        // 0x0x0x0x00x e/acc
        // Deriving the PDA for using makers pubKey and seeds
        let escrow = Pubkey::find_program_address(
            &[b"escrow".as_ref(), payer.pubkey().as_ref()],
            &PROGRAM_ID.parse().unwrap(),
        );
        msg!("Escrow PDA: {}\n", escrow.0);

        // 0x0x0x0x00x e/acc
        // Derive PDA for vault ATA
        let vault = spl_associated_token_account::get_associated_token_address(
            &SplPubkey::new_from_array(escrow.0.to_bytes()),
            &SplPubkey::new_from_array(mint_a.to_bytes()),
        );
        msg!("Vault PDA: {}\n", vault);

        // 0x0x0x0x00x e/acc
        // Define program IDs for ATAP, TP, SP, and other relevant information
        let associated_token_program = ASSOCIATED_TOKEN_PROGRAM_ID.parse::<Pubkey>().unwrap();
        let token_program = TOKEN_PROGRAM_ID;
        let system_program = solana_sdk_ids::system_program::ID;

        // 0x0x0x0x00x e/acc
        // Minting 1k tokens because I can.
        MintTo::new(&mut svm, &payer, &mint_a, &maker_ata_a, 1000000000)
            .send()
            .unwrap();

        let amount_to_receive: u64 = 100000000;
        let amount_to_give: u64 = 500000000;
        let bump: u8 = escrow.1;

        msg!("Bump: {}", bump);

        // 0x0x0x0x00x e/acc
        // Writing the Make transaction to deposit to the escrow
        let make_data = [
            vec![0u8], // this one is the discriminator: 8 bytes, remember?!
            bump.to_le_bytes().to_vec(),
            amount_to_receive.to_le_bytes().to_vec(),
            amount_to_give.to_le_bytes().to_vec(),
        ]
        .concat();
        let make_ix = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(mint_a, false),
                AccountMeta::new(mint_b, false),
                AccountMeta::new(escrow.0, false),
                AccountMeta::new(maker_ata_a, false),
                AccountMeta::new(Pubkey::new_from_array(vault.to_bytes()), false),
                AccountMeta::new(system_program, false),
                AccountMeta::new(token_program, false),
                AccountMeta::new(associated_token_program, false),
                AccountMeta::new(Rent::id(), false),
            ],
            data: make_data,
        };

        // 0x0x0x0x00x e/acc
        // Creating the message and sending
        let message = Message::new(&[make_ix], Some(&payer.pubkey()));
        let recent_blockhach = svm.latest_blockhash();
        let transaction = Transaction::new(&[payer], message, recent_blockhach);

        // 0x0x0x0x00x e/acc
        // Send ix and capture result
        let ix = svm.send_transaction(transaction).unwrap();

        // 0x0x0x0x00x e/acc
        // Logging details
        msg!("\n\nMake transaction successful");
        msg!("\nCUs Consumed: {}", ix.compute_units_consumed);
        println!("\n\n{}", ix.pretty_logs());
    }
}
