use {
    crate::{errors::StoreError, state::Key},
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo,
        borsh::try_from_slice_unchecked,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_instruction,
        sysvar::{rent::Rent, Sysvar},
    },
};

#[inline(always)]
pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> Result<(), ProgramError> {
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let required_lamport = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamport > 0 {
        invoke(
            &system_instruction::transfer(&payer_info.key, new_account_info.key, required_lamport),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    let accounts = &[new_account_info.clone(), system_program_info.clone()];

    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        accounts,
        &[&signer_seeds],
    )?;

    invoke_signed(
        &system_instruction::assign(new_account_info.key, &program_id),
        accounts,
        &[&signer_seeds],
    )?;
    Ok(())
}

pub fn assert_derivation(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(&path, program_id);
    if key != *account.key {
        return Err(StoreError::DerivedKeyInvalid.into());
    }
    Ok(bump)
}

pub fn try_from_slice_checked<T: BorshDeserialize>(
    data: &[u8],
    data_type: Key,
    data_size: usize,
) -> Result<T, ProgramError> {
    if (data[0] != data_type as u8 && data[0] != Key::Unitialized as u8) || data.len() != data_size
    {
        return Err(StoreError::DataTypeMismatch.into());
    }

    let result: T = try_from_slice_unchecked(data)?;
    Ok(result)
}
