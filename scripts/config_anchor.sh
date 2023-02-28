#!/bin/bash
set -e
#
export NEAR_ENV=testnet
export APPCHAIN_ID=barnacle-ci
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
export ANCHOR_ACCOUNT_ID=$APPCHAIN_ID'.'$REGISTRY_ACCOUNT_ID
export OCT_TOKEN_ACCOUNT_ID=oct.beta_oct_relay.testnet
export WRAPPED_TOKEN_ACCOUNT_ID=$APPCHAIN_ID.testnet
export TEST_OCT_ACCOUNT_ID=test_oct.testnet

# init anchor
near call $ANCHOR_ACCOUNT_ID new '{"appchain_id": "'$APPCHAIN_ID'", "appchain_registry": "'$REGISTRY_ACCOUNT_ID'", "oct_token": "'$OCT_TOKEN_ACCOUNT_ID'", "appchain_template_type":"Barnacle"}' --accountId $ANCHOR_ACCOUNT_ID
# set era reward 5
near call $ANCHOR_ACCOUNT_ID set_era_reward '{"era_reward":"5000000000000000000"}' --accountId $ANCHOR_ACCOUNT_ID
# set wrapped token
near call $ANCHOR_ACCOUNT_ID set_account_of_wrapped_appchain_token '{"contract_account": "'$WRAPPED_TOKEN_ACCOUNT_ID'"}' --accountId $ANCHOR_ACCOUNT_ID
near call $ANCHOR_ACCOUNT_ID set_metadata_of_wrapped_appchain_token '{"metadata": {"spec":"ft-1.0.0","name":"barnacle-ci token","symbol":"BARCI","decimals":18}}' --accountId $ANCHOR_ACCOUNT_ID
near call $ANCHOR_ACCOUNT_ID set_premined_balance_of_wrapped_appchain_token '{"premined_beneficiary":"oct-pallet-test.testnet","premined_balance":"10000000000000000000"}' --accountId $ANCHOR_ACCOUNT_ID
near call $ANCHOR_ACCOUNT_ID set_total_supply_of_wrapped_appchain_token '{"total_supply":"100000000000000000000"}' --accountId $ANCHOR_ACCOUNT_ID
# change minimum_validator_deposit_changing_amount 1 OCT
near call $ANCHOR_ACCOUNT_ID change_minimum_validator_deposit_changing_amount '{"value":"1000000000000000000"}' --accountId $ANCHOR_ACCOUNT_ID
# change minimum_validator_deposit 10 OCT
near call $ANCHOR_ACCOUNT_ID change_minimum_validator_deposit '{"value":"10000000000000000000"}' --accountId $REGISTRY_ACCOUNT_ID
# register alice-octopus.testnet as validator, 10 OCT
near call $OCT_TOKEN_ACCOUNT_ID ft_transfer_call '{"receiver_id": "'$ANCHOR_ACCOUNT_ID'", "amount": "10000000000000000000", "msg": "{\"RegisterValidator\": {\"profile\": {}, \"validator_id_in_appchain\": \"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d\", \"can_be_delegated_to\": false}}"}' --accountId alice-octopus.testnet --amount 0.000000000000000000000001 --gas 200000000000000
# register bob-octopus.testnet as validator, 10 OCT
near call $OCT_TOKEN_ACCOUNT_ID ft_transfer_call '{"receiver_id": "'$ANCHOR_ACCOUNT_ID'", "amount": "10000000000000000000", "msg": "{\"RegisterValidator\": {\"profile\": {}, \"validator_id_in_appchain\": \"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48\", \"can_be_delegated_to\": false}}"}' --accountId bob-octopus.testnet --amount 0.000000000000000000000001 --gas 200000000000000
near call $ANCHOR_ACCOUNT_ID set_token_price_maintainer_account '{"account_id": "'$TEST_OCT_ACCOUNT_ID'"}' --accountId $ANCHOR_ACCOUNT_ID
# set OCT price decimal=6 minimum_total_stake_price_for_booting: 10w U
near call $ANCHOR_ACCOUNT_ID set_price_of_oct_token '{"price": "6000000000"}' --accountId $TEST_OCT_ACCOUNT_ID
# change minimum_validator_count 2
near call $ANCHOR_ACCOUNT_ID change_minimum_validator_count '{"value":"2"}' --accountId $ANCHOR_ACCOUNT_ID
# move appchain to booting
near call $ANCHOR_ACCOUNT_ID generate_initial_validator_set --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000

# check the setting
near view $ANCHOR_ACCOUNT_ID get_anchor_settings
near view $ANCHOR_ACCOUNT_ID get_appchain_settings
near view $ANCHOR_ACCOUNT_ID get_protocol_settings

near view $ANCHOR_ACCOUNT_ID get_appchain_state
near view $ANCHOR_ACCOUNT_ID get_anchor_status

near view $ANCHOR_ACCOUNT_ID get_oct_token
near view $ANCHOR_ACCOUNT_ID get_wrapped_appchain_token