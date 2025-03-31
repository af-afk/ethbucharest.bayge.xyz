use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolCall},
    prelude::*,
    stylus_core::{calls::context::Call, log},
};

use crate::{errors::*, events, prover, storage_prover::*};

#[allow(unused)]
use alloc::vec::Vec;

sol!("./src/ICallback.sol");

#[cfg(feature = "contract-prover")]
#[public]
impl StorageProver {
    pub fn prove(&self, hash: FixedBytes<32>, from: u32) -> R<(u32, u32)> {
        Ok(prover::default_solve(hash.as_slice(), from).unwrap())
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
        let gas_starting = self.vm().evm_gas_left();
        let ICallback::proveReturn {
            lower: expect_lower,
            upper: expect_upper,
        } = ICallback::proveCall::abi_decode_returns(
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
        let gas_consumed = gas_starting - self.vm().evm_gas_left();
        let (our_lower, our_upper) =
            prover::default_solve(word.as_slice(), expect_lower).ok_or(Err::ProveLocalFailed)?;
        // If the remote results are inconsitent with ours, we need to indicate
        // there was an error here.
        if our_lower != expect_lower || our_upper != expect_upper {
            return Err(Err::ProveInconsistent);
        }
        log(
            self.vm(),
            events::Checked {
                addr: contract_addr,
                points: points_addr,
                gas: gas_consumed,
                word,
            },
        );
        // Check if the gas that was consumed is more than 1% of the top scorer.
        // If they are, then we update the top result, and we also log that we
        // did so.
        let last_top_scorer = self
            .top_scorers
            .get(self.top_scorers.len().checked_sub(1).unwrap_or(0))
            .map(unpack_result_word);
        match last_top_scorer {
            Some((last_score, _)) => {
                if last_score >= gas_consumed {
                    return Ok(());
                }
                let one_percent_of_prev = last_score / 100;
                let gas_delta = last_score - gas_consumed;
                // If the delta is more than 1%...
                if one_percent_of_prev <= gas_delta {
                    return Ok(());
                }
            }
            None => (),
        };
        // Wow! We found a winner! We need to emit, and update storage.
        log(
            self.vm(),
            events::NewWinner {
                addr: contract_addr,
                points: points_addr,
                gas: gas_consumed,
            },
        );
        if let Some((_, addr)) = last_top_scorer {
            let last_tenant_ts = u64::from_le_bytes(self.tenancy_start_ts.get().to_le_bytes());
            self.payout(last_tenant_ts, self.vm().block_timestamp(), addr)?;
        }
        self.tenancy_start_ts
            .set(U64::from(self.vm().block_timestamp()));
        self.top_scorers
            .push(pack_result_word(gas_consumed, points_addr));
        Ok(())
    }
}

impl StorageProver {
    // Payout the user some of the winning token that they should've earned
    // per second by multiplying it by the current rate of the payout on the
    // curve.
    pub fn payout(&mut self, deposited_when: u64, cur_time: u64, recipient: Address) -> R<U256> {
        // Calculate the amount per second they've earned using the curve
        // function, then multiply it by the seconds they've been in the lead.

        // Now, mint the tokens to send to the recipient.

        Ok(U256::ZERO)
    }
}
