use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instructions::StoreInstructions;

pub mod create_store;

pub struct Processor;
impl Processor {
    pub fn process_instruction<'a>(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = StoreInstructions::try_from_slice(instruction_data)?;

        match instruction {
            StoreInstructions::CreateStore => {
                msg!("Instruction: Create Store");
                create_store::process_create_store(program_id, accounts)
            }
            // StoreInstructions::UpdateStore => Self::process_update_store(accounts, program_id),
            StoreInstructions::DropStore => Self::process_drop_store(accounts, program_id),
        }
    }

    // fn process_update_store(_accounts: &[AccountInfo], _program_id: &Pubkey) -> ProgramResult {
    //     Ok(())
    // }

    // ToDo: to implenent this function after implementing the products functionality
    fn process_drop_store(_accounts: &[AccountInfo], _program_id: &Pubkey) -> ProgramResult {
        Ok(())
    }
}
