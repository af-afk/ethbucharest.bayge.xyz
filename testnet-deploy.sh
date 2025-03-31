#!/bin/sh -e

export \
	SPN_SUPERPOSITION_URL=https://testnet-rpc.superposition.so \
	SPN_PROVER_ADMIN=0xFEb6034FC7dF27dF18a3a6baD5Fb94C0D3dCb6d5

./deploy.sh
