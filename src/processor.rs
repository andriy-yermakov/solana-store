use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instructions::StoreInstructions;

pub mod create_store;
pub mod delete_store;

pub use create_store::*;
pub use delete_store::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StoreInstructions::try_from_slice(instruction_data)?;

    match instruction {
        StoreInstructions::CreateStore => {
            msg!("Instruction: Create Store");
            process_create_store(program_id, accounts)
        }
        StoreInstructions::DeleteStore => {
            msg!("Instruction: Delete Store");
            process_delete_store(accounts)
        }
    }
}
