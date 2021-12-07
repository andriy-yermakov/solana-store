#![cfg(feature = "test-bpf")]

use {
    solana_program::{borsh::try_from_slice_unchecked, pubkey::Pubkey},
    solana_program_test::*,
    solana_sdk::{
        hash::Hash,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    store::{instructions, processor, state::Key, state::Store, STORE_PREFIX},
};

async fn setup_store() -> (Pubkey, BanksClient, Pubkey, Keypair, Hash) {
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        "store",
        program_id,
        processor!(processor::process_instruction),
    );

    // Start executing test
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let payer_key = payer.pubkey();

    let store_seeds = &[
        STORE_PREFIX.as_bytes(),
        program_id.as_ref(),
        payer_key.as_ref(),
    ];

    let (store_key, _) = Pubkey::find_program_address(store_seeds, &program_id);

    // Create store account
    let tx = Transaction::new_signed_with_payer(
        &[instructions::create_create_store_instruction(
            program_id, store_key, payer_key, payer_key,
        )],
        Some(&payer_key),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    let store = banks_client.get_account(store_key).await.unwrap().unwrap();
    let store_data: Store = try_from_slice_unchecked(&store.data).unwrap();

    assert_eq!(store_data.key, Key::Store);
    assert_eq!(store_data.active, true);
    assert_eq!(store_data.admin_wallet, payer_key);

    return (program_id, banks_client, store_key, payer, recent_blockhash);
}

#[tokio::test]
async fn create_store_success() {
    setup_store().await;
}

#[tokio::test]
async fn delete_store_success() {
    let (program_id, mut banks_client, store_key, payer, recent_blockhash) = setup_store().await;

    let payer_key = payer.pubkey();

    // Delete store account
    let tx = Transaction::new_signed_with_payer(
        &[instructions::create_delete_store_instruction(
            program_id, store_key, payer_key,
        )],
        Some(&payer_key),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    let store = banks_client.get_account(store_key).await.unwrap();

    assert_eq!(store, None);
}
