#[cfg(any(feature = "contract-prover", feature = "factory-prover"))]
use alloc::{vec, vec::Vec};

use stylus_sdk::{
    alloy_primitives::{aliases::*, *},
    prelude::*,
    storage::*,
};

#[cfg_attr(
    any(feature = "contract-prover", feature = "factory-prover"),
    storage,
    entrypoint
)]
pub struct StorageProver {
    /// Admin that has the right to provide contract invocations to this function.
    pub admin: StorageAddress,

    /// Token to use to distribute points.
    pub token_addr: StorageAddress,

    /// The top scorer of this function, in the form of the address, and the
    /// gas consumed. Appended to, but occassionally scratched from the
    /// record by the administrator if someone misleads the address of their
    /// repo when they submit.
    pub top_scorers: StorageVec<StorageU256>,

    /// When the current winner begun their tenancy of the top position.
    pub tenancy_start_ts: StorageU64,

    /// Has the competition concluded?
    pub concluded: StorageBool,

    /// When this contract was deployed.
    pub started: StorageU64,

    /// When this competition concludes!
    pub deadline: StorageU64,
}

pub fn unpack_result_word(x: U256) -> (u64, Address) {
    let b: [u8; 32] = x.to_be_bytes();
    let addr = Address::from_slice(&b[12..]);
    let amt = u64::from_be_bytes(b[0..8].try_into().unwrap());
    (amt, addr)
}

pub fn pack_result_word(amt: u64, addr: Address) -> U256 {
    let mut b = [0u8; 32];
    b[0..8].copy_from_slice(&amt.to_be_bytes());
    b[12..].copy_from_slice(&addr.into_array());
    U256::from_be_bytes(b)
}

#[cfg(all(not(target_arch = "wasm32"), test))]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_unpack_results(amt in any::<u64>(), addr in any::<[u8; 20]>()) {
            let addr = Address::from(addr);
            assert_eq!(
                (amt, addr),
                unpack_result_word(pack_result_word(amt, addr))
            );
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for StorageProver {
    fn default() -> Self {
        use stylus_sdk::testing::vm::TestVM;
        StorageProver::from(&TestVM::new())
    }
}