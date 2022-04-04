#!/bin/bash
TOKEN_NAME=BmiEarlyAccess
TOKEN_TICKET=BMI
# Deployment
erdpy --verbose contract deploy --chain="D" --bytecode="output/bmi-token.wasm" --pem="alice.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send --outfile=./scripts/bmi_token.json

CONTRACT_ADDRESS=`cat scripts/bmi_token.json | jq -r '.contractAddress'`

# Interaction
erdpy --verbose contract call ${CONTRACT_ADDRESS} --pem="alice.pem" --gas-limit=600000000 --function="issueTokens" --value=05000 --arguments str:${TOKEN_NAME} str:${TOKEN_TICKET} --proxy="https://devnet-gateway.elrond.com" --chain=D --recall-nonce --send
sleep 10
erdpy --verbose contract call $CONTRACT_ADDRESS --pem="alice.pem" --gas-limit=2000000 --function="setLocalRoles" --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send
sleep 10
erdpy --verbose contract call $CONTRACT_ADDRESS --pem="alice.pem" --gas-limit=2000000 --function="mintTokens" --arguments 1000000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send
sleep 10
