use crayfish_program::{program_error::ProgramError, pubkey::Pubkey, RawAccountInfo, Ref};

use crate::{FromAccountInfo, ReadableAccount};

pub struct UncheckedAccount<'a> {
    info: &'a RawAccountInfo,
}

impl<'a> FromAccountInfo<'a> for UncheckedAccount<'a> {
    fn try_from_info(info: &'a RawAccountInfo) -> Result<Self, ProgramError> {
        Ok(UncheckedAccount { info })
    }
}

impl<'a> AsRef<RawAccountInfo> for UncheckedAccount<'a> {
    fn as_ref(&self) -> &RawAccountInfo {
        self.info
    }
}

impl<'a> ReadableAccount for UncheckedAccount<'a> {
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