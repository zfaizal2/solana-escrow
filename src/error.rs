use thiserror::Error;

use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

///defining error type
#[derive(Error,Debug,Copy,Clone)]
pub enum EscrowError {
    ///invalid instruction
    #[error("InvalidInstruction")]
    InvalidInstruction,
    ///not rent exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    ///Expected amount mismatch,
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    ///overflow
    #[error("Amount Overflow")]
    AmountOverflow,
}   

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}