use stylus_sdk::alloy_sol_types::{sol, SolError};

use alloc::vec::Vec;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Err {
    AdminOnly,

    /// The caller to the prove function reverted!
    ProveRevert,

    /// When we went to unpack the prove, we weren't able to!
    ProveUnpack,

    /// The local prove program failed to complete when given remote results.
    ProveLocalFailed,

    /// The results of the local prove against the other contract's prove are inconsistent.
    ProveInconsistent,
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
