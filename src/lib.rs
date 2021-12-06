use solana_program;

pub mod entrypoint;
pub mod errors;
pub mod instructions;
pub mod processor;
pub mod state;
pub mod utils;

pub const STORE_PREFIX: &str = "store";

solana_program::declare_id!("Qu6JfKnqdHJQpZ775JSdR8iqhHXXseRR8qSxNKgKAxj");
