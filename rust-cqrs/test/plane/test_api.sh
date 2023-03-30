#!/bin/bash

RANDOM=$$
TEST_ACCT="test-acct-$RANDOM"
TEST_URL="localhost:3030/plane/$TEST_ACCT"

echo "Using test plane: $TEST_ACCT"
echo "Registering an plane"

curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"Openplane\": {\"plane_id\": \"$TEST_ACCT\"}}"
echo "Depositing money"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data "@test/plane/DepositMoney.json"
echo "Withdrawing money"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data "@test/plane/WithdrawMoney.json"
echo "Writing a check"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data "@test/plane/WriteCheck.json"
echo "Checking plane status (calling a query)"
curl -i --location $TEST_URL
echo
