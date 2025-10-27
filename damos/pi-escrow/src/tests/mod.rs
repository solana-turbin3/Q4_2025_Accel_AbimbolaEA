#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use litesvm::LiteSVM;
    use litesvm_token::{
        spl_token::{
            self, 
        }, CreateAssociatedTokenAccount, CreateMint, MintTo};
    use pinocchio_pubkey::derive_address;
    // use pinocchio::{msg, program};
    use solana_instruction::{AccountMeta, Instruction};
    use solana_keypair::Keypair;
    use solana_program::{
                msg, rent::Rent, sysvar::SysvarId
            };
    use solana_message::Message;
    use solana_native_token::LAMPORTS_PER_SOL;
    use solana_transaction::Transaction;
    use solana_pubkey::Pubkey;
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
        svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).expect("Airdrop failed");

    // Loading the .so file to the runtime
    // 0x0x0x0x0x0x00xMem e/acc Cracked by design

    msg!("The path is!! {}", env!("CARGO_MANIFEST_DIR"));
    let so_path = PathBuf::from("/home/eaa/accel_builders/repo/damos/pi-escrow/target/sbf-solana-solana/release/pi_escrow.so");
    msg!("The path is {:?}", so_path);

    msg!("The program ID is: {:?}", crate::id());

    let program_data = std::fs::read(so_path).expect("Failed to read program SO file");
    svm.add_program(program_id(), &program_data);
    (svm, payer)
    }

    #[test]
    pub fn test_make_instruction() {
        let(mut svm, payer) = setup();
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

        // 0xABIMBOLA +++++++++++++
        // Creating unique ATAs for the subs
        let maker_ata_a = CreateAssociatedTokenAccount::new(&mut svm, &payer, &mint_a)
            .owner(&payer.pubkey()).send().unwrap();
        msg!("Maker ATA A: {}\n", maker_ata_a);

        // 0xABIMBOLA +++++++++++++
        // Deriving the PDA for using makers pubKey and seeds
        let escrow = Pubkey::find_program_address(
            &[b"escrow".as_ref(), payer.pubkey().as_ref()],
            &PROGRAM_ID.parse().unwrap(),
        );
        msg!("Escrow PDA: {}\n", escrow.0);

        // 0xABIMBOLA +++++++++++++
        // Derive PDA for vault ATA
        // let vault = spl_associated_token_account::get_associated_token_address(&program_id, &mint_a);

        let vault = derive_address(
            &[b"escrow".as_ref(), payer.pubkey().as_ref()], Some(255), &program_id.to_bytes());
        msg!("Vault PDA: {:?}\n", vault);

        // 0xABIMBOLA +++++++++++++
        // Define program IDs for ATAP, TP, SP, and other relevant information
        let associated_token_program = ASSOCIATED_TOKEN_PROGRAM_ID.parse::<Pubkey>().unwrap();
        let token_program = TOKEN_PROGRAM_ID;
        let system_program = solana_sdk_ids::system_program::ID;

        // 0xABIMBOLA +++++++++++++
        // Minting 1k tokens because I can.
        MintTo::new(&mut svm, &payer, &mint_a, &maker_ata_a, 1000000000)
            .send()
            .unwrap();

        let amount_to_receive: u64 = 100000000;
        let amount_to_give: u64 = 500000000;
        let bump: u8 = escrow.1;

        msg!("Bump: {}", bump);
        
        // 0xABIMBOLA +++++++++++++
        // Writing the Make transaction to deposit to the escrow
        let make_data = [
            vec![0u8], // this one is the discriminator: 8 bytes, remember?!
            bump.to_le_bytes().to_vec(),
            amount_to_receive.to_le_bytes().to_vec(),
            amount_to_give.to_le_bytes().to_vec(),
        ].concat();
        let make_ix = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(mint_a, false),
                AccountMeta::new(mint_b, false),
                AccountMeta::new(escrow.0, false),
                AccountMeta::new(maker_ata_a, false),
                AccountMeta::new(vault.into(), false),
                AccountMeta::new(system_program, false),
                AccountMeta::new(token_program, false),
                AccountMeta::new(associated_token_program, false),
                AccountMeta::new(Rent::id(), false),
            ],
            data: make_data, 
        };

        // 0xABIMBOLA +++++++++++++
        // Creating the message and sending
        let message = Message::new(&[make_ix], Some(&payer.pubkey()));
        let recent_blockhach = svm.latest_blockhash();
        let transaction = Transaction::new(&[payer], message, recent_blockhach);

        // 0xABIMBOLA +++++++++++++
        // Send ix and capture result
        let ix = svm.send_transaction(transaction).unwrap();

        // 0xABIMBOLA +++++++++++++
        // Logging details
        msg!("\n\nMake transaction successful");
        msg!("\nCUs Consumed: {}", ix.compute_units_consumed);
        println!("\n\n{}", ix.pretty_logs());
    }

    #[test]
    pub fn test_take_instruction() {
        let(mut svm, payer) = setup();
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
        msg!("Mint B: {}\n", mint_b);

        let escrow = Pubkey::find_program_address(
            &[b"escrow".as_ref(), payer.pubkey().as_ref()],
            &PROGRAM_ID.parse().unwrap(),
            );
            msg!("Escrow PDA: {:?}\n", escrow.1);

        let taker_ata = CreateAssociatedTokenAccount::new(
            &mut svm, 
            &payer, &mint_b)
                .owner(&payer.pubkey()).send().unwrap();
            msg!("Taker ATA: {}", taker_ata);

        let vault = spl_associated_token_account::get_associated_token_address(
            &SplPubkey::new_from_array((escrow.0.to_bytes())),
            &SplPubkey::new_from_array(mint_b.to_bytes())
        );
        msg!("Vault PDA: {}\n", vault);


        let associated_token_program = ASSOCIATED_TOKEN_PROGRAM_ID.parse::<Pubkey>().unwrap();
        let token_program = TOKEN_PROGRAM_ID;
        let system_program = solana_sdk_ids::system_program::ID;

         MintTo::new(&mut svm, &payer, &mint_b, &taker_ata, 1000000000)
            .send()
            .unwrap();

        let amount_to_receive: u64 = 900000000;
        let amount_to_give: u64 = 700000000;
        let bump: u8 = escrow.1;

        msg!("Bump: {}", bump);

        // 0xABIMBOLA +++++++++++++
        // Writing the Take transaction to withdraw from the Escrow if conditions ae met

        let take_data = [
            vec![0u8], // 8 for the discriminator
            bump.to_le_bytes().to_vec(),
            amount_to_give.to_le_bytes().to_vec(),
            amount_to_receive.to_le_bytes().to_vec(),
        ].concat();

        let take_ix = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(mint_a, false),
                AccountMeta::new(mint_b, false),
                AccountMeta::new(escrow.0, false),
                AccountMeta::new(Pubkey::new_from_array((vault.to_bytes())), false),
                AccountMeta::new(taker_ata, false),
                AccountMeta::new(token_program, false),
                AccountMeta::new(system_program, false),
                AccountMeta::new(associated_token_program, false),
            ],
            data: take_data,
        };

        // 0xABIMBOLA +++++++++++++
        // Creating the take message and sending ix
        let message = Message::new(&[take_ix], Some(&payer.pubkey()));
        let recent_blockhash = svm.latest_blockhash();
        let transaction = Transaction::new(&[payer], message, recent_blockhash);

        let ix = svm.send_transaction(transaction).unwrap();
        // 0x0x0x0x0x0x00xMem e/acc
        // Logging details
        msg!("\n\nTake Transaction successful");
        msg!("\nCUs Consumed: {}", ix.compute_units_consumed);
        println!("\n\n More information {:?}", ix.pretty_logs());
    }
}