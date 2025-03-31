#!/bin/sh

log() {
	>&2 echo $@
}

err() {
	log $@
	exit 1
}

[ -z "$SPN_SUPERPOSITION_KEY" ] && err "SPN_SUPERPOSITION_KEY unset"
[ -z "$SPN_PROVER_ADMIN" ] && err "SPN_PROVER_ADMIN unset"

if [ -z "$SPN_PROVER_IMPL" ]; then
	export SPN_PROVER_IMPL="$(./deploy-stylus.sh contract-prover.wasm)"
fi

log "SPN_PROVER_IMPL=$SPN_PROVER_IMPL"

if [ -z "$SPN_PROVER_FACTORY" ]; then
	export SPN_PROVER_FACTORY="$(./deploy-stylus.sh factory-prover.wasm)"
fi

log "SPN_PROVER_FACTORY=$SPN_PROVER_FACTORY"

export \
	RAW_PRIVATE_KEY=$SPN_SUPERPOSITION_KEY \
	ETH_RPC_URL=$SPN_SUPERPOSITION_URL

cast send \
	--rpc-url "$SPN_SUPERPOSITION_URL" \
	--private-key "$SPN_SUPERPOSITION_KEY" \
	"$SPN_PROVER_FACTORY" \
		'deploy(address,address)' \
		"$SPN_PROVER_IMPL" \
		"$SPN_PROVER_ADMIN"
