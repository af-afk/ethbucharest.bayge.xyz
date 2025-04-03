#[cfg(feature = "contract-prover")]
use {
    crate::prover,
    alloc::{string::String, vec::Vec},
};

use crate::{errors::*, events, immutables::*, storage_prover::*};

use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolCall},
    prelude::*,
    stylus_core::{calls::context::Call, log},
};

#[cfg(feature = "contract-prover")]
sol!("./src/ICallback.sol");

sol! {
    function mint(address recipient, uint256 amount);
}

#[cfg(feature = "contract-prover")]
#[public]
impl StorageProver {
    pub fn register(&self, contract_addr: Address, points_addr: Address, info: String) -> R<()> {
        log(
            self.vm(),
            events::Registered {
                addr: contract_addr,
                recipient: points_addr,
                info,
            },
        );
        Ok(())
    }

    pub fn admin(&self) -> Address {
        self.admin.get()
    }

    pub fn token_addr(&self) -> Address {
        self.token_addr.get()
    }

    pub fn prove(&self, hash: FixedBytes<32>, from: u32) -> R<(u32, u32)> {
        Ok(prover::default_solve(hash.as_slice(), from).unwrap())
    }

    pub fn check(
        &mut self,
        contract_addr: Address,
        word: FixedBytes<32>,
        points_addr: Address,
    ) -> R<u64> {
        // To prevent word golfing abuse, this function is only usable by the admin.
        if self.vm().msg_sender() != self.admin.get() {
            return Err(Err::AdminOnly);
        }
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
        self.internal_update_leader(contract_addr, points_addr, gas_consumed)?;
        Ok(gas_consumed)
    }

    /// Cancel the latest submission to the contract, assuming we're not
    /// having a million submissions at once, and only taking place if the
    /// address of the latest sender is the victim specified. This might be
    /// need to be triggered if there's abuse taking place (perhaps a user
    /// didn't share the repository).
    pub fn cancel(&mut self, victim: Address) -> R<()> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(Err::AdminOnly);
        }
        let (_, addr) = unpack_result_word(self.top_scorers.pop().ok_or(Err::CancelInappropriate)?);
        if addr != self.admin.get() {
            return Err(Err::AdminOnly);
        }
        log(self.vm(), events::CancelTookPlace { victim });
        Ok(())
    }

    /// Conclude the competition, sending the winner their token.
    pub fn conclude(&mut self) -> R<()> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(Err::AdminOnly);
        }
        if self.concluded.get() {
            return Err(Err::AlreadyConcluded);
        }
        self.internal_maybe_payout_cur_leader()?;
        log(self.vm(), events::Concluded {});
        self.concluded.set(true);
        Ok(())
    }

    pub fn token_amount_owed(&self) -> R<U256> {
        if let Some(_) = self.internal_get_last_top_winner() {
            Ok(self.internal_tokens_owed_cur_leader())
        } else {
            Ok(U256::ZERO)
        }
    }

    pub fn current_leader_solution(&self) -> R<(u64, Address)> {
        Ok(
            if let Some((points, addr)) = self.internal_get_last_top_winner() {
                (points, addr)
            } else {
                (0, Address::ZERO)
            },
        )
    }

    pub fn upgrade(&mut self, impl_: Address) -> R<()> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(Err::AdminOnly);
        }
        // Just in case during this competition... This is the slot impl impl
        let w = U256::from_limbs([
            2351945876927687612,
            14573121138821903785,
            461557562180745613,
            3893525298072888097,
        ]);
        unsafe {
            self.vm().storage_cache_bytes32(w, impl_.into_word());
        }
        Ok(())
    }
}

impl StorageProver {
    pub fn internal_get_last_top_winner(&self) -> Option<(u64, Address)> {
        self.top_scorers
            .get(self.top_scorers.len().checked_sub(1).unwrap_or(0))
            .map(unpack_result_word)
    }

    pub fn internal_update_leader(
        &mut self,
        contract_addr: Address,
        points_addr: Address,
        gas_consumed: u64,
    ) -> R<()> {
        // Check if the gas that was consumed is more than 1% of the top scorer.
        // If they are, then we update the top result, and we also log that we
        // did so.
        match self.internal_get_last_top_winner() {
            Some((last_score, _)) => {
                if last_score <= gas_consumed {
                    return Ok(());
                }
                let one_percent_of_prev = last_score / 100;
                let gas_delta = last_score - gas_consumed;
                // If the delta is more than 1%...
                if one_percent_of_prev >= gas_delta {
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
        self.internal_maybe_payout_cur_leader()?;
        self.tenancy_start_ts
            .set(U64::from(self.vm().block_timestamp()));
        self.top_scorers
            .push(pack_result_word(gas_consumed, points_addr));
        Ok(())
    }

    pub fn internal_mint(&self, recipient: Address, amt_owed: U256) -> R<()> {
        let _ = self
            .vm()
            .call(
                &Call::new(),
                self.token_addr.get(),
                &mintCall {
                    recipient,
                    amount: amt_owed,
                }
                .abi_encode(),
            )
            .map_err(|x| match x {
                calls::errors::Error::Revert(_) => Err::MintFailed,
                _ => unimplemented!(),
            })?;
        Ok(())
    }

    pub fn internal_challenge_duration(&self) -> u64 {
        let x = self.deadline.get() - self.started.get();
        u64::from_be_bytes(x.to_be_bytes())
    }

    pub fn internal_tokens_owed(&self, deposited_when: u64, cur_time: u64) -> U256 {
        U256::from(cur_time - deposited_when) * BASE_REVENUE
    }

    pub fn internal_tokens_owed_cur_leader(&self) -> U256 {
        let last_tenant_ts = u64::from_le_bytes(self.tenancy_start_ts.get().to_le_bytes());
        // Make sure we don't overflow what's left of the deadline and have some
        // weird funny business with the final amount distribution.
        let max_time = {
            let ts = self.vm().block_timestamp();
            let d = u64::from_be_bytes(self.deadline.get().to_be_bytes());
            if ts > d {
                d
            } else {
                ts
            }
        };
        self.internal_tokens_owed(last_tenant_ts, max_time)
    }

    pub fn internal_maybe_payout_cur_leader(&mut self) -> R<()> {
        if let Some((_, addr)) = self.internal_get_last_top_winner() {
            let tokens_owed = self.internal_tokens_owed_cur_leader();
            self.internal_mint(addr, tokens_owed)?;
            log(
                self.vm(),
                events::WinnerPaidOut {
                    leader: addr,
                    amount: tokens_owed,
                },
            );
        }
        Ok(())
    }
}
