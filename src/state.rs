use borsh::{BorshDeserialize, BorshSerialize};
use {
    crate::utils::try_from_slice_checked,
    solana_program::{
        account_info::AccountInfo, program_error::ProgramError, program_pack::Sealed,
        pubkey::Pubkey,
    },
};

pub const MAX_STORE_SIZE: usize = 1 + // key
1 + // active
32; // admin_wallet

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug, Copy)]
pub enum Key {
    Unitialized,
    Store,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug)]
pub struct Store {
    pub key: Key,
    pub active: bool,
    pub admin_wallet: Pubkey,
}

impl Sealed for Store {}

impl Store {
    pub fn from_account_info(a: &AccountInfo) -> Result<Store, ProgramError> {
        let store: Store =
            try_from_slice_checked(&a.data.borrow_mut(), Key::Store, MAX_STORE_SIZE)?;

        Ok(store)
    }
}
