#!/bin/bash
set -e
#
export NEAR_ENV=testnet
export APPCHAIN_ID=barnacle-ci
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
export ANCHOR_ACCOUNT_ID=$APPCHAIN_ID'.'$REGISTRY_ACCOUNT_ID

near view $ANCHOR_ACCOUNT_ID get_anchor_status > result.txt
export END_ERA=`grep 'index_range_of_validator_set_history' result.txt | awk -F"end_index: '" '{print $2}' | awk -F"'" '{print $1}'`
echo "END_ERA: "$END_ERA
rm -f result.txt

for ((i=$END_ERA;i>=0;i--))
do
    param="'{\"era_number\":\"${i}\"}'"
    near call $ANCHOR_ACCOUNT_ID clear_reward_distribution_records $param --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
    near call $ANCHOR_ACCOUNT_ID clear_unwithdrawn_rewards $param --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
    near call $ANCHOR_ACCOUNT_ID remove_validator_set_history_of $param --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
done
#
near call $ANCHOR_ACCOUNT_ID clear_unbonded_stakes '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_next_validator_set '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_user_staking_histories '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_staking_histories '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_validator_profiles '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_appchain_messages '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_appchain_notification_histories '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_external_assets_registration '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID remove_staged_wasm '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_validator_set_histories '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID clear_contract_level_lazy_option_values '' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
#
near call $ANCHOR_ACCOUNT_ID remove_storage_keys '{"keys":["U1RBVEU=","bm50","b2N0","d2F0"]}' --accountId $ANCHOR_ACCOUNT_ID --gas 200000000000000
# delete anchor account Do you want to proceed? (y/n)
# near delete $ANCHOR_ACCOUNT_ID $REGISTRY_ACCOUNT_ID
# https://explorer.testnet.near.org/transactions/FMSSxEJZpLJSnWV8feyZ4CpNkLhGr2DJnRuv9a4UgicR