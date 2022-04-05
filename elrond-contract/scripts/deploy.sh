#!/bin/bash
TOKEN_NAME=BmiEarlyAccess
TOKEN_TICKET=BMI
# Deployment
erdpy contract build
erdpy --verbose contract deploy --chain="D" --bytecode="output/bmi-token.wasm" --pem="alice.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send --outfile=./scripts/bmi_token.json

# shellcheck disable=SC2006
# shellcheck disable=SC2002
CONTRACT_ADDRESS=`cat scripts/bmi_token.json | jq -r '.contractAddress'`
sleep 20
# Interaction
erdpy --verbose contract call "${CONTRACT_ADDRESS}" --pem="alice.pem" --gas-limit=80000000 --function="issueTokens" --value=50000000000000000 --arguments str:${TOKEN_NAME} str:${TOKEN_TICKET} --proxy="https://devnet-gateway.elrond.com" --chain=D --recall-nonce --send
sleep 30
erdpy --verbose contract call "$CONTRACT_ADDRESS" --pem="alice.pem" --gas-limit=100000000 --function="setLocalRoles" --proxy="https://devnet-gateway.elrond.com" --chain="D" --recall-nonce --send
sleep 20
erdpy --verbose contract call "$CONTRACT_ADDRESS" --pem="alice.pem" --gas-limit=300000000 --function="mintTokens" --arguments 1000000000 --proxy="https://devnet-gateway.elrond.com" --chain="D" --recall-nonce --send
sleep 20
