#!/bin/bash
set -e
#
export NEAR_ENV=testnet
export APPCHAIN_ID=barnacle-ci
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
export ANCHOR_ACCOUNT_ID=$APPCHAIN_ID'.'$REGISTRY_ACCOUNT_ID
export WRAPPED_TOKEN_ACCOUNT_ID=$APPCHAIN_ID.testnet
export TEST_ACCOUNT_ID=oct-pallet-test.testnet

# deploy wrapped token, execute only once! 
# near deploy --accountId $WRAPPED_TOKEN_ACCOUNT_ID --wasmFile out/main.wasm
# init wrapped token, execute only once! 
# near call $WRAPPED_TOKEN_ACCOUNT_ID new '{"owner_id": "'$ANCHOR_ACCOUNT_ID'","premined_beneficiary":"'$TEST_ACCOUNT_ID'","premined_balance":"100000000000000000000","metadata":{"spec":"ft-1.0.0","name":"barnacle-ci token","symbol":"BARCI","decimals":18}}' --accountId $WRAPPED_TOKEN_ACCOUNT_ID --gas 200000000000000
