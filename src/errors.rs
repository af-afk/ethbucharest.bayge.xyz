use stylus_sdk::{
    alloy_sol_types::{sol, SolError},
    stylus_core::calls::errors::Error,
};

use alloc::vec::Vec;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Err {
    AdminOnly,
    ProveRevert,
    ProveUnpack,
}

impl From<Err> for u8 {
    fn from(v: Err) -> Self {
        v as u8
    }
}

pub type R<T> = Result<T, Err>;

sol! {
    error Revert(uint8);
}

impl From<Err> for Vec<u8> {
    fn from(v: Err) -> Self {
        Revert { _0: v.into() }.abi_encode()
    }
}
