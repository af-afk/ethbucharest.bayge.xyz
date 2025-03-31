use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolCall},
    prelude::deploy::CachePolicy,
};

#[allow(unused)]
use stylus_sdk::prelude::*;

#[allow(unused)]
use alloc::vec::Vec;

use crate::{errors::*, proxy, storage_prover::*};

sol! {
    function setup(address _admin);
}

#[cfg(feature = "factory-prover")]
#[public]
impl StorageProver {
    pub fn deploy(&self, contract_impl: Address, admin: Address) -> R<Address> {
        let mut c = proxy::metaphor_proxy_code(contract_impl).to_vec();
        let mut setup_cd = setupCall { _admin: admin }.abi_encode().to_vec();
        c.append(&mut setup_cd);
        unsafe {
            self.vm()
                .deploy(&c, U256::ZERO, None, CachePolicy::Flush)
                .map_err(|_| Err::DeployFailed)
        }
    }

    /// Callback function that's used to set the storage of the prover contract.
    pub fn setup(&mut self, admin: Address) -> R<()> {
        self.admin.set(admin);
        /*
        let t = unsafe {
            self.vm()
                .deploy(&proxy::POINTS_TOKEN_BYTECODE, U256::ZERO, None, CachePolicy::Flush)
                .map_err(|_| Err::DeployFailed)?
        };
        self.token_addr.set(t);
        */
        Ok(())
    }
}
