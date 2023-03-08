#!/bin/bash
set -e
#
export NEAR_ENV=testnet
export APPCHAIN_ID=barnacle-ci
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
export ANCHOR_ACCOUNT_ID=$APPCHAIN_ID'.'$REGISTRY_ACCOUNT_ID
export OCT_TOKEN_ACCOUNT_ID=oct.beta_oct_relay.testnet

# create anchor account, execute only once!
# near create-account $ANCHOR_ACCOUNT_ID --masterAccount $REGISTRY_ACCOUNT_ID --publicKey ed25519:99dtM6c33a1NCoRrhM7cwfDaJy123JLdsT9i4M9wYk3f --initialBalance 25
# deploy anchor, execute only once!
# near deploy --accountId $ANCHOR_ACCOUNT_ID --wasmFile res/appchain_anchor.wasm
# storage_deposit for oct token, execute only once! 
# near call $OCT_TOKEN_ACCOUNT_ID storage_deposit '{"account_id": "'$ANCHOR_ACCOUNT_ID'", "registration_only": null}' --accountId $ANCHOR_ACCOUNT_ID --amount 0.1
# https://explorer.testnet.near.org/transactions/CkQ611cPBeUVMAXWqnoWAYAUFng6bGm8PAttc8EaoLc2