use {
    crate::{errors::StoreError, state::Store},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
    },
};

pub fn delete_store_logic<'a>(
    store_info: &AccountInfo<'a>,
    admin_wallet_info: &AccountInfo<'a>,
) -> ProgramResult {
    if store_info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let store = Store::from_account_info(store_info)?;

    if false == store.active {
        return Err(StoreError::StoreNotActive.into());
    }

    if store.admin_wallet != *admin_wallet_info.key {
        return Err(ProgramError::IllegalOwner);
    }

    let balance = store_info.lamports();

    if balance > 0 {
        **store_info.try_borrow_mut_lamports()? -= balance;
        **admin_wallet_info.try_borrow_mut_lamports()? += balance;
    }

    Ok(())
}

pub fn process_delete_store(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let store_info = next_account_info(account_info_iter)?;
    let admin_wallet_info = next_account_info(account_info_iter)?;

    delete_store_logic(store_info, admin_wallet_info)?;

    Ok(())
}
