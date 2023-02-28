#!/bin/bash
set -e

./target/release/appchain-barnacle \
    --base-path /tmp/alice \
    --chain local \
    --alice \
    --port 30333 \
    --ws-port 9945 \
    --rpc-port 9933 \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    --enable-offchain-indexing true \
    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    --validator