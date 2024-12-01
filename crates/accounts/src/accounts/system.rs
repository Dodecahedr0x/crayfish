use crayfish_errors::Error;
use crayfish_program::{
    program_error::ProgramError, pubkey::Pubkey, system_program, RawAccountInfo, Ref,
};

use crate::{FromAccountInfo, ReadableAccount};

pub struct SystemAccount<'a> {
    info: &'a RawAccountInfo,
}

impl<'a> FromAccountInfo<'a> for SystemAccount<'a> {
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        if info.owner() != &system_program::ID {
            return Err(Error::InvalidOwner {
                wanted: system_program::ID,
                current: *info.owner(),
            }
            .into());
        }

        Ok(SystemAccount { info })
    }
}

impl<'a> AsRef<RawAccountInfo> for SystemAccount<'a> {
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<'a> ReadableAccount for SystemAccount<'a> {
    type DataType = [u8];

    fn key(&self) -> &Pubkey {
        self.info.key()
    }

    fn owner(&self) -> &Pubkey {
        self.info.owner()
    }

    fn lamports(&self) -> Result<Ref<u64>, ProgramError> {
        self.info.try_borrow_lamports()
    }

    fn data(&self) -> Result<Ref<Self::DataType>, ProgramError> {
        self.info.try_borrow_data()
    }
}
