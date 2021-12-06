use {
    crate::{
        errors::StoreError,
        state::{Key, Store},
        utils::assert_derivation,
        STORE_PREFIX,
    },
    borsh::BorshSerialize,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_instruction,
    },
};

pub fn delete_store_logic<'a>(
    program_id: &Pubkey,
    _store_program_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    store_info: &AccountInfo<'a>,
    admin_wallet_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
) -> ProgramResult {
    if store_info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    if !payer_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut store = Store::from_account_info(store_info)?;

    if false == store.active {
        return Err(StoreError::StoreNotActive.into());
    }

    if store.admin_wallet != *admin_wallet_info.key {
        return Err(ProgramError::IllegalOwner);
    }

    store.key = Key::Store;
    store.active = false;
    // ToDo: to fill admin_wallet field by zeros
    // store.admin_wallet = Pubkey::new(&[0u8; 32]);
    store.serialize(&mut *store_info.data.borrow_mut())?;

    let lamports = store_info.lamports();
    if lamports > 0 {
        // transfer lamports to admn_wallet_info
        let store_bump = assert_derivation(
            program_id,
            store_info,
            &[
                STORE_PREFIX.as_bytes(),
                program_id.as_ref(),
                admin_wallet_info.key.as_ref(),
            ],
        )?;

        let signer_seeds: &[&[u8]] = &[
            STORE_PREFIX.as_bytes(),
            program_id.as_ref(),
            admin_wallet_info.key.as_ref(),
            &[store_bump],
        ];
        invoke_signed(
            &system_instruction::transfer(store_info.key, admin_wallet_info.key, lamports),
            &[
                store_info.clone(),
                admin_wallet_info.clone(),
                system_info.clone(),
            ],
            &[&signer_seeds],
        )?;
    }

    Ok(())
}

pub fn process_delete_store(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let store_info = next_account_info(account_info_iter)?;
    let admin_wallet_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let store_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    delete_store_logic(
        program_id,
        store_program_info,
        system_info,
        store_info,
        admin_wallet_info,
        payer_info,
    )?;
    Ok(())
}
