#!/bin/sh

cargo test --features contract-prover -- print_tokens_owed --nocapture
