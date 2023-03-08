#!/bin/bash
set -e
#
export NEAR_ENV=testnet
export APPCHAIN_ID=barnacle-ci
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
export ANCHOR_ACCOUNT_ID=$APPCHAIN_ID'.'$REGISTRY_ACCOUNT_ID
export OCT_TOKEN_ACCOUNT_ID=oct.beta_oct_relay.testnet
export WRAPPED_TOKEN_ACCOUNT_ID=$APPCHAIN_ID.testnet

# set gateway endpoint ws://127.0.0.1:9945
near call $ANCHOR_ACCOUNT_ID set_rpc_endpoint '{"rpc_endpoint": "ws://127.0.0.1:9945"}' --accountId $ANCHOR_ACCOUNT_ID
# set relayer ci-relayer.testnet
near call $ANCHOR_ACCOUNT_ID set_relayer_account '{"account_id": "ci-relayer.testnet"}' --accountId $ANCHOR_ACCOUNT_ID
# set beefy keys
near call $ANCHOR_ACCOUNT_ID initialize_beefy_light_client '{"initial_public_keys": ["0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1","0x0390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f27"]}' --accountId $ANCHOR_ACCOUNT_ID
# enable witness mode before beefy is online
near call $ANCHOR_ACCOUNT_ID turn_on_beefy_light_client_witness_mode --accountId $ANCHOR_ACCOUNT_ID
# go live
near call $ANCHOR_ACCOUNT_ID go_live --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000

# check the setting
near view $ANCHOR_ACCOUNT_ID get_anchor_version

near view $ANCHOR_ACCOUNT_ID get_anchor_settings
near view $ANCHOR_ACCOUNT_ID get_appchain_settings
near view $ANCHOR_ACCOUNT_ID get_protocol_settings

near view $ANCHOR_ACCOUNT_ID get_appchain_state
near view $ANCHOR_ACCOUNT_ID get_anchor_status

near view $ANCHOR_ACCOUNT_ID get_oct_token
near view $ANCHOR_ACCOUNT_ID get_wrapped_appchain_token