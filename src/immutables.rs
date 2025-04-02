use stylus_sdk::alloy_primitives::U256;

pub const BOARD_SIZE: u32 = 0x1FFFF;

pub const MAX_TRIES: u32 = 10_000;

pub const CHECKS_NEEDED: u32 = 2;

pub const DEFAULT_DEADLINE: u64 = 0;

pub const BASE_REVENUE: U256 = U256::from_limbs([1000000000000000000, 0, 0, 0]);
