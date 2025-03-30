
use stylus_sdk::prelude::*;

#[cfg_attr(feature = "factory-prover", entrypoint)]
impl StorageProver {
    pub fn deploy(&self, admin: Address) -> R<Address> {
        metaphoric_proxy::deploy(admin)
    }
}
