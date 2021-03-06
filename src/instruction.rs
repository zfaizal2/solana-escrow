
use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;


pub enum EscrowInstruction {
    /// create and populate escrow account & transfer ownership of temp token to program derived address

    /// 5 accounts in total as params
    ///
    /// [signer] - acct of person init escrow
    /// [writable] - temp token acct, owned by initializer (should already exist)
    /// [] - initializer's token acct for the token they will receive
    /// [writable] for escrow account
    /// [] - rent sysvar
    /// [] - token program
    /// 
    /// [] is read-only
    /// [writable] changes data field

    /// api endpoint to start escrow
    InitEscrow {
        //amount A expects 
        amount: u64
    },
    Exchange {
        /// the amount the taker expects to be paid in the other token, as a u64 because that's the max possible supply of a token
        amount: u64,
    }
}

impl EscrowInstruction {

    ///unpack looks at first byte to determine how to decode
    pub fn unpack(input :&[u8]) -> Result<Self, ProgramError> {
        //splitting instruction, else throw invalid
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        

        //match tag input to instruction, either 0 or _
        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Exchange {
                amount: Self::unpack_amount(rest)?
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }


    /// decodes 'rest' to get a u64 representing the amount
    /// choose which instruction to build, then builds and returns that instruction
    pub fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}

