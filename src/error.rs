use thiserror::Error;
use crate::{instruction::EscrowInstruction, error::EscrowError};

use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

///defining error type
#[derive(Error,Debug,Copy,Clone)]
pub enumb EscrowError {
    ///invalid instruction
    #[error("InvalidInstruction")]
    InvalidInstruction,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> {
        ProgramError::Custom(e as u32)
    }
}