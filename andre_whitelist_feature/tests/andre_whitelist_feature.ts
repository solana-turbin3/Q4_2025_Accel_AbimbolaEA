import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  createInitializeMintInstruction,
  ExtensionType,
  getMintLen,
  createAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  createTransferCheckedInstruction,
  createTransferCheckedWithTransferHookInstruction,
  createInitializeTransferHookInstruction
} from "@solana/spl-token";
import { sendAndConfirmRawTransaction, sendAndConfirmTransaction, SendTransactionError, SystemProgram, Transaction } from "@solana/web3.js"
import { AndreWhitelistFeature } from "../target/types/andre_whitelist_feature";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("whitelist-transfer-hooks", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.AndreWhitelistFeature as Program<AndreWhitelistFeature>;
  const mint2022 = anchor.web3.Keypair.generate();

  //sender acct token
  const sourceTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_PROGRAM_ID,
  )
  //receiveracct token
  const recipient = anchor.web3.Keypair.generate();
  const destinationTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    recipient.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_PROGRAM_ID,
  )

  const [extraAccountMetaListPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('extra-account-metas'), mint2022.publicKey.toBuffer()],
    program.programId,
  );

  const whitelist = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("whitelist")],
    program.programId
  )[0];

  it("Initializes the Whitelist", async () => {
    const tx = await program.methods.initializeWhitelist()
    .accountsPartial({
      admin: provider.publicKey,
      whitelist,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    console.log("\n Whitelist Initialized:", whitelist.toBase58());
    console.log("\n Transaction signature:", tx);
  });

  it("Add user to whitelist", async () => {
    const tx = await program.methods.addToWhitelist(provider.publicKey)
    .accountsPartial({
      admin: provider.publicKey,
      whitelist,
    }).rpc();

    console.log("\n User added to whitelist: ", provider.publicKey.toBase58());
    console.log("\n Transaction signature: ", tx);
  });

  it("Remove user from whitelist", async () => {
    const tx = await program.methods.removeFromWhitelist(provider.publicKey)
    .accountsPartial({
      admin: provider.publicKey,
      whitelist,
    }).rpc();

    console.log("\n User Removed from whitelist: ", provider.publicKey.toBase58());
    console.log("\n Transaction signature: ", tx);
  });

  it('Create Mint account with Transfer Hook Extension' , async () => {
    const extensions = [ExtensionType.TransferHook];
    const mintLen = getMintLen(extensions);
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);
    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mint2022.publicKey,
        space: mintLen,
        lamports: lamports,
        programId: TOKEN_2022_PROGRAM_ID
      }),
      createInitializeTransferHookInstruction(
        mint2022.publicKey,
        wallet.publicKey,
        program.programId,
        TOKEN_2022_PROGRAM_ID,
      ),
      createInitializeMintInstruction(
        mint2022.publicKey,
        9,
        wallet.publicKey,
        null,
        TOKEN_2022_PROGRAM_ID,
      ));
    const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer, mint2022], {
        skipPreflight: true,
        commitment: 'finalized',
      });

    const txDetails = await program.provider.connection.getTransaction(txSig, {
        maxSupportedTransactionVersion: 0,
        commitment: 'finalized',
      });

      console.log("\n Transaction Signature: ", txSig);
  });

  it('Create Token Accounts and Mint Tokens', async () => {
    const amount = 100 * 10 ** 9;
    const transaction = new Transaction().add(
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        sourceTokenAccount,
        wallet.publicKey,
        mint2022.publicKey,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_PROGRAM_ID
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        destinationTokenAccount,
        recipient.publicKey,
        mint2022.publicKey,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_PROGRAM_ID,
      ),
      createMintToInstruction(mint2022.publicKey, sourceTokenAccount, wallet.publicKey, amount, [], TOKEN_2022_PROGRAM_ID),
    );
    const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer], {
      skipPreflight: true
    });
    console.log("\n Transaction Signature: ", txSig);
  });

  it('Create ExtraAccountMetaList Account', async() => {
    const initializeExtraAccountMetaListInstruction = await program.methods
    .initializeTransferHook()
    .accountsPartial({
      payer: wallet.publicKey,
      mint: mint2022.publicKey,
      extraAccountMetaList: extraAccountMetaListPDA,
      systemProgram: SystemProgram.programId,
    }).instruction();

    const transaction = new Transaction().add(initializeExtraAccountMetaListInstruction);

    const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer], {
      skipPreflight: true,
      commitment: 'confirmed'
    });
    console.log("\n ExtraAccountMetaList Account Created:", extraAccountMetaListPDA.toBase58());
    console.log("Transaction Signature: ", txSig);
  });

  it('Transfer Hook with Extra Account Meta', async () => {
    const amount = 1 * 10 ** 9;
    const amountBigInt = BigInt(amount);
    const transferInstructionWithHelper = await createTransferCheckedWithTransferHookInstruction(
      provider.connection,
      sourceTokenAccount,
      mint2022.publicKey,
      destinationTokenAccount,
      wallet.publicKey,
      amountBigInt,
      9,
      [],
      'confirmed',
      TOKEN_2022_PROGRAM_ID
    );
    const transaction = new Transaction().add(transferInstructionWithHelper);
    try{
      const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer], {skipPreflight: false});
      console.log("\n Transfer Signature: " , txSig);
    }

    catch (error) {
      if (error instanceof SendTransactionError) {
        console.error("\n Transaction failed: ", error.logs[5]);
      } else {
        console.error("\n Unexpected error: ", error);
      }
    }
  });
});
