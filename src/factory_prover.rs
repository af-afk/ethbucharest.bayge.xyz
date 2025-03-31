use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolCall},
};

#[allow(unused)]
use stylus_sdk::prelude::*;

#[allow(unused)]
use alloc::vec::Vec;

use crate::{errors::*, events, proxy, storage_prover::*};

#[cfg(feature = "factory-prover")]
#[public]
impl StorageProver {
    pub fn deploy(&self, contract_impl: Address, admin: Address) -> R<Address> {
        let mut c = proxy::metaphor_proxy_code(contract_impl).to_vec();
        let mut admin_addr = [0u8; 32];
        admin_addr[12..].copy_from_slice(&admin.into_array());
        c.extend_from_slice(&mut admin_addr);
        let addr = unsafe {
            self.vm()
                .deploy(&c, U256::ZERO, None)
                .map_err(|_| Err::DeployFailed)
        }?;
        log(self.vm(), events::Deployed { deployment: addr });
        Ok(addr)
    }

    /// Callback function that's used to set the storage of the prover contract.
    pub fn setup(&mut self, admin: Address) -> R<()> {
        self.admin.set(admin);
        let t = unsafe {
            self.vm()
                .deploy(&proxy::POINTS_TOKEN_BYTECODE, U256::ZERO, None)
                .map_err(|_| Err::DeployFailed)?
        };
        self.token_addr.set(t);
        Ok(())
    }
}
