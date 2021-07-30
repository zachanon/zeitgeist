#!/usr/bin/env bash

# See `testing-network.sh`.

export BOOTNODES=""
export DATA_DIR="$HOME/battery-station-relay"
export DOCKER_POLKADOT_BIN="/usr/local/bin/polkadot"
export PARACHAINS_NUM="1"
export VALIDATORS_NUM="3"

export PARACHAIN="zeitgeist-battery-station-relay-parachain"
export PARACHAIN_CHAIN="battery_station"
export PARACHAIN_ID="2050"
export PARACHAIN_IMAGE="zeitgeistpm/zeitgeist-node-parachain:sha-8cf9df0"
export PARACHAIN_PORT="30000"
export PARACHAIN_RPC_PORT="8000"
export PARACHAIN_WS_PORT="9000"

export VALIDATOR="zeitgeist-battery-station-relay-validator"
export VALIDATOR_IMAGE="zeitgeistpm/zeitgeist-relay-chain:sha-73e221e"
export VALIDATOR_PORT="31000"
export VALIDATOR_RPC_PORT="8100"
export VALIDATOR_WS_PORT="9100"

export RELAY_CHAIN_SPEC_FILE="/tmp/relay-chain-spec.json"
curl -o $RELAY_CHAIN_SPEC_FILE https://raw.githubusercontent.com/zeitgeistpm/polkadot/battery-station-relay/node/service/res/battery-station-relay.json

. "$(dirname "$0")/testing-network.sh" --source-only