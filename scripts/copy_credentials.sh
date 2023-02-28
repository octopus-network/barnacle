#!/bin/bash
set -e

echo $ANCHOR_PK > ~/.near-credentials/testnet/barnacle-ci.registry.test_oct.testnet.json
echo $WRAPPED_TOKEN_PK > ~/.near-credentials/testnet/barnacle-ci.testnet.json
echo $REGISTRY_PK > ~/.near-credentials/testnet/registry.test_oct.testnet.json
echo $ALICE_VALIDATOR_PK > ~/.near-credentials/testnet/alice-octopus.testnet.json
echo $BOB_VALIDATOR_PK > ~/.near-credentials/testnet/bob-octopus.testnet.json
echo $TEST_OCT_PK > ~/.near-credentials/testnet/test_oct.testnet.json