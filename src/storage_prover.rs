use stylus_sdk::{
    alloy_primitives::{aliases::*, *},
    storage::*,
};

#[allow(unused)]
use {
    alloc::{vec, vec::Vec},
    stylus_sdk::prelude::*,
};

#[storage]
#[entrypoint]
pub struct StorageProver {
    /// Admin that has the right to provide contract invocations to this function.
    pub admin: StorageAddress,

    /// The top scorer of this function, in the form of the address, and the
    /// gas consumed. Appended to, but occassionally scratched from the
    /// record by the administrator if someone misleads the address of their
    /// repo when they submit.
    pub top_scorers: StorageVec<StorageU256>,

    /// Amounts of seconds of tenancy the leading solution has had, which is converted into
    /// a token that's minted and sent their way on change.
    pub tenancy_secs: StorageU64,
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