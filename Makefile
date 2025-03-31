
RELEASE_WASM := target/wasm32-unknown-unknown/release/libbucharesthashing.wasm

CARGO_BUILD_STYLUS := \
	cargo build \
		--release \
		--target wasm32-unknown-unknown \
		--features

build: contract-prover.wasm factory-prover.wasm

.PHONY: build

contract-prover.wasm: $(shell find src -type f -name '*.rs')
	@rm -f contract-prover.wasm
	@${CARGO_BUILD_STYLUS} contract-prover
	@cp ${RELEASE_WASM} contract-prover.wasm

factory-prover.wasm: $(shell find src -type f -name '*.rs')
	@rm -f factory-prover.wasm
	@${CARGO_BUILD_STYLUS} factory-prover
	@cp ${RELEASE_WASM} factory-prover.wasm
