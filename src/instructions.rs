use {
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
    /// 3. `[]` System
    /// 4. `[]` Rent sysvar
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
    /// 0. `[writable]` The store key
    /// 1. `[signer]` The admin wallet
    DeleteStore,
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
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(rent::id(), false),
    ];
    Instruction {
        program_id,
        accounts,
        data: StoreInstructions::CreateStore.try_to_vec().unwrap(),
    }
}

pub fn create_delete_store_instruction(
    program_id: Pubkey,
    store: Pubkey,
    admin: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(store, false),
        AccountMeta::new_readonly(admin, true),
    ];
    Instruction {
        program_id,
        accounts,
        data: StoreInstructions::DeleteStore.try_to_vec().unwrap(),
    }
}
