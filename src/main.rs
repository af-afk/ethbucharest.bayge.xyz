#![cfg_attr(target_arch = "wasm32", no_main, no_std)]

#[panic_handler]
#[cfg(target_arch = "wasm32")]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
