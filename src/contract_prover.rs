use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolCall},
    prelude::*,
    stylus_core::calls::context::Call,
};

use crate::{errors::*, storage_prover::*};

#[allow(unused)]
use alloc::vec::Vec;

sol!("./src/ICallback.sol");

#[public]
impl StorageProver {
    pub fn register(&self, contract_addr: Address, points_recipient: Address) -> R<()> {
        Ok(())
    }

    pub fn prove(&self, hash: FixedBytes<32>, from: u32) -> R<(u32, u32)> {
        Ok(crate::prover::solve(0x1FFFFFFF, 1, 100000, hash.as_slice(), from).unwrap())
    }

    pub fn check(
        &mut self,
        contract_addr: Address,
        word: FixedBytes<32>,
        points_addr: Address,
    ) -> R<()> {
        // Check the contract's performance, by taking the random word, then
        // supplying it as an argument to the contract given by having the gas
        // amount estimated beforehand, then measuring the impact on the gas
        // remaining at the end of the function. So the recorded gas amount is
        // the delta between the gas supplied to this call, and the amount that's
        // remaining.
        let cur_gas = self.vm().evm_gas_left();
        let ICallback::proveReturn { lower, upper } = ICallback::proveCall::abi_decode_returns(
            &self
                .vm()
                .static_call(
                    &Call::new(),
                    contract_addr,
                    &ICallback::proveCall {
                        hash: word,
                        from: 0,
                    }
                    .abi_encode(),
                )
                .map_err(|_| Err::ProveRevert)?,
            true,
        )
        .map_err(|_| Err::ProveUnpack)?;
        let gas_remaining = self.vm().evm_gas_left();
        Ok(())
    }
}
