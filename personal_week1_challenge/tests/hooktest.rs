use anchor_lang::prelude::*;
use litesvm::LiteSVM;
use litesvm_token::{
    get_spl_account,
    spl_token_2022::CreateAssociatedTokenAccount, CreateMint, TransferHook
};
use solana_sdk::signature::{Keypair, Signer,};

#[test]
fn test() {
    let mut svm = LiteSVM::new();
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();
    let balance = svm.get_balance(&user.pubkey()).unwrap();
    assert_eq(balance, 1_000_000_000);

    msg!("Account funded with {} SOL", balance as f64 / 1e9);
}