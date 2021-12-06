use {
    crate::id,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
        sysvar::rent,
    },
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum StoreInstructions {
    /// Create new store
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable]` The store pda key
    /// 1. `[signer]` The admin wallet
    /// 2. `[signer]` Payer
    /// 3. `[]` The store program
    /// 4. `[]` System
    /// 5. `[]` Rent sysvar
    CreateStore,

    /// Update store information
    ///
    ///
    /// Account expected:
    ///
    /// 0. `[signer]` The account of store's admin
    /// 1. `[writable]` The account of the store
    // UpdateStore,

    /// Drop existing store
    ///
    ///
    /// Account expected:
    ///
    /// 0, `[signer]` The account of store's admin
    /// 1. `[writable]` The account of the store
    DropStore,
}

pub fn create_create_store_instruction(
    program_id: Pubkey,
    store: Pubkey,
    admin: Pubkey,
    payer: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(store, false),
        AccountMeta::new_readonly(admin, true),
        AccountMeta::new_readonly(payer, true),
        AccountMeta::new_readonly(id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(rent::id(), false),
    ];
    Instruction {
        program_id,
        accounts,
        data: StoreInstructions::CreateStore.try_to_vec().unwrap(),
    }
}
