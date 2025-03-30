use stylus_sdk::alloy_sol_types::{sol, SolError};

use alloc::vec::Vec;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Err {
    AdminOnly,
}

impl From<Err> for u8 {
    fn from(v: Err) -> Self {
        v as u8
    }
}

pub type R<T> = Result<T, Err>;

sol! {
    error Error(uint8);
}

impl From<Err> for Vec<u8> {
    fn from(v: Err) -> Self {
        Error { _0: v.into() }.abi_encode()
    }
}
