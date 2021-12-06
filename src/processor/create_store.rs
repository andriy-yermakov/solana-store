use {
    crate::{
        state::{Key, Store, MAX_STORE_SIZE},
        utils::{assert_derivation, create_or_allocate_account_raw},
        STORE_PREFIX,
    },
    borsh::BorshSerialize,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

pub fn create_store_logic<'a>(
    program_id: &Pubkey,
    _store_program_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    store_info: &AccountInfo<'a>,
    admin_wallet_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
) -> ProgramResult {
    if !admin_wallet_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !payer_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let store_bump = assert_derivation(
        program_id,
        store_info,
        &[
            STORE_PREFIX.as_bytes(),
            program_id.as_ref(),
            admin_wallet_info.key.as_ref(),
        ],
    )?;
    if store_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            store_info,
            rent_info,
            system_info,
            payer_info,
            MAX_STORE_SIZE,
            &[
                STORE_PREFIX.as_bytes(),
                program_id.as_ref(),
                admin_wallet_info.key.as_ref(),
                &[store_bump],
            ],
        )?;
    }

    let mut store = Store::from_account_info(store_info)?;
    store.key = Key::Store;
    store.active = true;
    store.admin_wallet = *admin_wallet_info.key;
    store.serialize(&mut *store_info.data.borrow_mut())?;
    Ok(())
}

pub fn process_create_store(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let store_info = next_account_info(account_info_iter)?;
    let admin_wallet_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let store_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    create_store_logic(
        program_id,
        store_program_info,
        rent_info,
        system_info,
        store_info,
        admin_wallet_info,
        payer_info,
    )?;
    Ok(())
}
