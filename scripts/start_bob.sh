#!/bin/bash
set -e

./target/release/appchain-barnacle \
    --base-path /tmp/bob \
    --chain local \
    --bob \
    --port 30334 \
    --ws-port 9946 \
    --rpc-port 9934 \
    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    --enable-offchain-indexing true \
    --validator \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp