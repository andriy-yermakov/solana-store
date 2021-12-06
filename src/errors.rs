use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum StoreError {
    /// Derived key invalid
    #[error("Derived key invalid")]
    DerivedKeyInvalid,
    /// Data type mismatch
    #[error("Data type mismatch")]
    DataTypeMismatch,
}

impl PrintProgramError for StoreError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<StoreError> for ProgramError {
    fn from(e: StoreError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for StoreError {
    fn type_of() -> &'static str {
        "Store Error"
    }
}
