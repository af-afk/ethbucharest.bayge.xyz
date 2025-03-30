#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

pub mod errors;
pub mod events;
pub mod prover;

pub mod storage_prover;

mod contract_prover;

pub use storage_prover::*;

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8) {
    use core::slice;
    use tiny_keccak::{Hasher, Keccak};
    let mut hasher = Keccak::v256();
    let data = unsafe { slice::from_raw_parts(bytes, len) };
    hasher.update(data);
    let output = unsafe { slice::from_raw_parts_mut(output, 32) };
    hasher.finalize(output);
}
