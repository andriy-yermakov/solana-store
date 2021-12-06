#![cfg(feature = "test-bpf")]
use {
    solana_program::{borsh::try_from_slice_unchecked, pubkey::Pubkey},
    solana_program_test::{tokio, ProgramTest},
    solana_sdk::{signature::Signer, transaction::Transaction},
    store::{instructions, state::Key, state::Store, STORE_PREFIX},
};

mod create_store {
    use super::*;

    pub fn program_test() -> ProgramTest {
        ProgramTest::new("store", store::id(), None)
    }

    #[tokio::test]
    async fn create_store_success() {
        let mut context = program_test().start_with_context().await;
        let payer_key = context.payer.pubkey();
        let mid = store::id();
        let store_seeds = &[STORE_PREFIX.as_bytes(), mid.as_ref(), payer_key.as_ref()];
        let (store_key, _) = Pubkey::find_program_address(store_seeds, &mid);

        let tx = Transaction::new_signed_with_payer(
            &[instructions::create_create_store_instruction(
                mid, store_key, payer_key, payer_key,
            )],
            Some(&payer_key),
            &[&context.payer],
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();
        let store = context
            .banks_client
            .get_account(store_key)
            .await
            .unwrap()
            .unwrap();
        let store_data: Store = try_from_slice_unchecked(&store.data).unwrap();
        assert_eq!(store_data.key, Key::Store);
        assert_eq!(store_data.active, true);
        assert_eq!(store_data.admin_wallet, payer_key);
    }

    // #[tokio::test]
    // async fn create_store_failure() {
    //     let mut context = program_test().start_with_context().await;
    //     let payer_key = context.payer.pubkey();
    //     let hack_payer = Keypair::new();
    //     let hack_payer_pub = hack_payer.pubkey();
    //     let mid = store::id();
    //     let store_seeds = &[STORE_PREFIX.as_bytes(), mid.as_ref(), payer_key.as_ref()];
    //     let hack_store_seeds = &[
    //         STORE_PREFIX.as_bytes(),
    //         mid.as_ref(),
    //         hack_payer_pub.as_ref(),
    //     ];
    //     let (store_key, _) = Pubkey::find_program_address(store_seeds, &mid);
    //     let (hack_store_key, _) = Pubkey::find_program_address(hack_store_seeds, &mid);
    //     let tx = Transaction::new_signed_with_payer(
    //         &[instructions::create_create_store_instruction(
    //             mid, store_key, payer_key, payer_key,
    //         )],
    //         Some(&payer_key),
    //         &[&context.payer],
    //         context.last_blockhash,
    //     );
    //     let result = context
    //         .banks_client
    //         .process_transaction(tx)
    //         .await
    //         .unwrap_err();
    // }
}
