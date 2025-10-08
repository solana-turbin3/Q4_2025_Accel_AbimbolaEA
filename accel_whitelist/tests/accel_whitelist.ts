import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccelWhitelist } from "../target/types/accel_whitelist";
import { PublicKey, Keypair, SystemProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import {
  TOKEN_2022_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  createInitializeMintInstruction,
  getMintLen,
  ExtensionType,
  createInitializeTransferHookInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
} from "@solana/spl-token";

describe("accel_whitelist", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.accelWhitelist as Program<AccelWhitelist>;

  const mintKeypair = anchor.web3.Keypair.generate();

  // User token account address
  const userTokenAccount = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  // ExtraAccountMetaList address
  const [extraAccountMetaList] = PublicKey.findProgramAddressSync(
    [Buffer.from('extra-account-metas'), mintKeypair.publicKey.toBuffer()],
    program.programId,
  );

  it("Create Mint Account with Transfer Hook Extension", async () => {
    const extensions = [ExtensionType.TransferHook];
    const mintLen = getMintLen(extensions);
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);

    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKeypair.publicKey,
        space: mintLen,
        lamports: lamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferHookInstruction(
        mintKeypair.publicKey,
        wallet.publicKey,
        program.programId, // Transfer Hook Program ID
        TOKEN_2022_PROGRAM_ID,
      ),
      createInitializeMintInstruction(mintKeypair.publicKey, 9, wallet.publicKey, null, TOKEN_2022_PROGRAM_ID),
    );

    const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer, mintKeypair], {
      skipPreflight: true,
      commitment: 'finalized',
    });

    console.log("\nMint created with transfer hook:", mintKeypair.publicKey.toBase58());
    console.log("Transaction signature:", txSig);
  });

  it('Create ExtraAccountMetaList Account', async () => {
    const initializeExtraAccountMetaListInstruction = await program.methods
      .initializeTransferHook()
      .accountsPartial({
        user: wallet.publicKey,
        mint: mintKeypair.publicKey,
        extraAccountMetaList,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .instruction();

    const transaction = new Transaction().add(initializeExtraAccountMetaListInstruction);

    const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer], { skipPreflight: true, commitment: 'confirmed' });
    console.log("\nExtraAccountMetaList Account created:", extraAccountMetaList.toBase58());
    console.log('Transaction Signature:', txSig);
  });

  it("Initialize whitelist for user", async () => {
    const user = wallet.publicKey;
    const [whitelist] = PublicKey.findProgramAddressSync(
      [Buffer.from("whitelist"), user.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initializeWhitelist()
      .accountsPartial({
        admin: wallet.publicKey,
        user,
        whitelist,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("\nWhitelist initialized for user:", user.toBase58());
    console.log("Transaction signature:", tx);
  });

  it("Claim tokens and whitelist user", async () => {
    const user = wallet.publicKey;
    const [whitelist] = PublicKey.findProgramAddressSync(
      [Buffer.from("whitelist"), user.toBuffer()],
      program.programId
    );

    const amount = new anchor.BN(1000000000); // 1 token

    const tx = await program.methods
      .claimAndWhitelist(amount)
      .accountsPartial({
        user,
        mint: mintKeypair.publicKey,
        userTokenAccount,
        whitelist,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("\nTokens claimed and user whitelisted");
    console.log("Transaction signature:", tx);
  });
});
