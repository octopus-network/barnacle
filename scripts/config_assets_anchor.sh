#!/bin/bash
set -e
#
export NEAR_ENV=testnet
export APPCHAIN_ID=barnacle-ci
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
export ANCHOR_ACCOUNT_ID=$APPCHAIN_ID'.'$REGISTRY_ACCOUNT_ID
export ASSET_ACCOUNT_ID=oct.beta_oct_relay.testnet

# register near asset
near call $ANCHOR_ACCOUNT_ID register_near_fungible_token '{"symbol": "OCT", "name": "OCT Token", "decimals": 18, "contract_account": "'$ASSET_ACCOUNT_ID'", "price": "1000000"}' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
# open bridge for near asset
near call $ANCHOR_ACCOUNT_ID open_bridging_of_near_fungible_token '{"symbol": "OCT"}' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
# storage_deposit for near asset, execute only once! 
# near call $ASSET_ACCOUNT_ID storage_deposit '{"account_id": "'$ANCHOR_ACCOUNT_ID'", "registration_only": null}' --accountId $ANCHOR_ACCOUNT_ID --amount 0.1
# https://explorer.testnet.near.org/transactions/2AMmfMa84VJtvu2w6hMHQhefHvnQrCqXjQxb4wHqPT4P