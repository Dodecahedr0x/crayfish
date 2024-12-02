use crayfish_program::{
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use num_traits::{FromPrimitive, ToPrimitive};
use thiserror::Error;

/// Maybe rework with thiserror 2.0
#[derive(Debug, Error)]
pub enum Error {
    #[error("Program is not executable")]
    InvalidProgramExecutable,

    #[error("Account is initialized yet")]
    AccountNotInitialized,

    // #[error("The owner of the account is not {wanted}, currently it's {current}")]
    #[error("The current owner of the account does not match the expected one")]
    InvalidOwner { wanted: Pubkey, current: Pubkey },

    #[error("The given account is not mutable")]
    AccountNotMutable,

    #[error("Account is not a signer")]
    AccountNotSigner,

    #[error("The current owner of this account is not the expected one")]
    AccountOwnedByWrongProgram,

    #[error("Failed to deserialize account data")]
    CannotDeserializeData,
}

impl FromPrimitive for Error {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            3000 => Some(Error::InvalidProgramExecutable),
            3001 => Some(Error::AccountNotInitialized),
            3002 => Some(Error::InvalidOwner {
                current: [0; PUBKEY_BYTES],
                wanted: [0; PUBKEY_BYTES],
            }),
            3003 => Some(Error::AccountNotMutable),
            3004 => Some(Error::AccountNotSigner),
            3005 => Some(Error::AccountOwnedByWrongProgram),
            3006 => Some(Error::CannotDeserializeData),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}

impl ToPrimitive for Error {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Error::InvalidProgramExecutable => Some(3000),
            Error::AccountNotInitialized => Some(3001),
            Error::InvalidOwner {
                current: _,
                wanted: _,
            } => Some(3002),
            Error::AccountNotMutable => Some(3003),
            Error::AccountNotSigner => Some(3004),
            Error::AccountOwnedByWrongProgram => Some(3005),
            Error::CannotDeserializeData => Some(3006),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_i64().map(|n| n as u64)
    }
}

impl From<Error> for ProgramError {
    fn from(value: Error) -> Self {
        msg!("[ERROR] {}", value.to_string());
        ProgramError::Custom(value.to_u32().unwrap())
    }
}
