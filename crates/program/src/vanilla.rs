pub use solana_nostd_entrypoint::solana_program::*;
pub use solana_nostd_entrypoint::{Ref, RefMut};

pub use nostd_system_program as system_program;

pub mod sysvars {
    pub use solana_nostd_entrypoint::solana_program::sysvar::*;
}

pub type RawAccountInfo = solana_nostd_entrypoint::NoStdAccountInfo;
pub type Account = solana_nostd_entrypoint::AccountInfoC;
pub type Instruction = solana_nostd_entrypoint::InstructionC;
pub type Signer<'a, 'b> = &'a [&'b [u8]];
