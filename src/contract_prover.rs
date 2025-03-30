use stylus_sdk::{abi, alloy_primitives::*};

use crate::{errors::*, storage_prover::*};

use alloc::{vec::Vec};

#[cfg_attr(
    any(feature = "contract-prover", feature = "factory-prover"),
    stylus_sdk::prelude::public
)]
impl StorageProver {
    pub fn register(&self, contract_addr: Address, points_recipient: Address) -> R<()> {
        Ok(())
    }

    pub fn prove(&self, hash: abi::Bytes, from: u32) -> R<(u32, u32)> {
        Ok(crate::prover::solve(0x1FFFFFFF, 1, 100000, &hash, from).unwrap())
    }
}
