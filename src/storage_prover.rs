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

    /// Results that were supplied to this function, aka the median of the registrations that
    /// were provided to this contract. The word contains an address, and the gas performance of
    /// the function was called, in the form of the address, and the gas as a
    /// u160 word, needing to be decoded below.
    pub results: StorageVec<StorageU256>,

    /// Amounts of seconds of tenancy the leading solution has had, which is converted into
    /// a token that's minted and sent their way on change.
    pub tenancy_secs: u64,
}

pub fn unpack_result_word(x: U256) -> (U96, Address) {
    let b: [u8; 32] = x.to_be_bytes();
    let addr = Address::from_slice(&b[12..32]);
    let amt = U96::from_be_bytes::<12>(b[0..12].try_into().unwrap());
    (amt, addr)
}

pub fn pack_result_word(amt: U256, addr: Address) -> U256 {
    let mut b = [0u8; 32];
    b[0..12].copy_from_slice(&amt.to_be_bytes::<32>()[20..]);
    b[12..32].copy_from_slice(addr.as_slice());
    U256::from_be_bytes(b)
}

#[cfg(all(not(target_arch = "wasm32"), test))]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn pack_unpack_queue_item(amt in any::<[u8; 12]>(), addr in any::<[u8; 20]>()) {
            let mut amt_u = [0u8; 32];
            amt_u[20..].copy_from_slice(&amt);
            let amt_u = U256::from_be_bytes(amt_u);
            let amt = U96::from_be_bytes(amt);
            let addr = Address::from(addr);
            assert_eq!(
                (amt, addr),
                unpack_result_word(pack_result_word(amt_u, addr)
            ));
        }
    }
}
