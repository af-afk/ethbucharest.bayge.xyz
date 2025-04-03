#[cfg(feature = "factory-prover")]
use {
    crate::{errors::*, events, immutables::*, proxy, storage_prover::*},
    alloc::vec::Vec,
    stylus_sdk::{alloy_primitives::*, prelude::*},
};

#[cfg(feature = "factory-prover")]
#[public]
impl StorageProver {
    pub fn deploy(&self, contract_impl: Address, admin: Address) -> Result<Address, Vec<u8>> {
        let c = proxy::metaphor_proxy_code(contract_impl, admin).to_vec();
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
        assert!(!admin.is_zero());
        self.admin.set(admin);
        let t = unsafe {
            self.vm()
                .deploy(&proxy::POINTS_TOKEN_BYTECODE, U256::ZERO, None)
                .map_err(|_| Err::DeployFailed)?
        };
        self.token_addr.set(t);
        self.started.set(U64::from(self.vm().block_timestamp()));
        self.deadline.set(U64::from(DEFAULT_DEADLINE));
        Ok(())
    }
}
