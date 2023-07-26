#!/usr/bin/env bash
set -euo pipefail

# Public key for the seed `//0`.
export NODE_PUBKEY=5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH
export BASE_PATH=/data/${NODE_PUBKEY}

aleph-node bootstrap-node --base-path ${BASE_PATH} --account-id ${NODE_PUBKEY} --chain-type local > /dev/null
export BOOTNODE_PEER_ID=$(aleph-node key inspect-node-key --file /data/${NODE_PUBKEY}/p2p_secret)

export ALLOW_PRIVATE_IPV4=true
export CHAIN=/data/chainspec.json
export CUSTOM_ARGS=-lerror
export DISCOVER_LOCAL=true
export NAME=AlephNode
export NODE_KEY_PATH=${BASE_PATH}/p2p_secret
export PORT=${PORT:-30333}
export PURGE_BEFORE_START=true
export RPC_PORT=${RPC_PORT:-9933}
export RUST_LOG=info
export UNIT_CREATION_DELAY=300
export WS_PORT=${WS_PORT:-9944}
export BOOT_NODES=/ip4/0.0.0.0/tcp/30333/p2p/${BOOTNODE_PEER_ID}
export PUBLIC_ADDR=/ip4/0.0.0.0/tcp/30333
export VALIDATOR_PORT=${VALIDATOR_PORT:-30343}
export PUBLIC_VALIDATOR_ADDRESS=0.0.0.0:30343

export PURGE_BEFORE_START=false

aleph-node bootstrap-chain \
  --raw \
  --base-path /data \
  --account-ids "$NODE_PUBKEY" \
  --chain-type local \
  > /data/chainspec.json

# `./docker_entrypoint.sh` comes from the original image and is the proper entrypoint
./docker_entrypoint.sh
