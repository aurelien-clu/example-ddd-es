#!/bin/bash

RANDOM=$$
TEST_ACCT="test-acct-$RANDOM"
TEST_URL="localhost:3030/plane/$TEST_ACCT"

echo "******************************"
echo "Using test plane: $TEST_ACCT"
echo "******************************"

echo "Registering an plane"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"Register\": {\"registration_id\": \"$TEST_ACCT\"}}"

echo "\nUpdating position"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"UpdatePosition\": {\"latitude\": 1.0, \"longitude\": 2.0, \"altitude\": 0 }}"

echo "\nPrepare for take off!"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"TakeOff\": null}"

echo "\nUpdating position"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"UpdatePosition\": {\"latitude\": 10.0, \"longitude\": 20.0, \"altitude\": 10 }}"

echo "\nJourney"
curl -i --location $TEST_URL

echo "\nUpdating position"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"UpdatePosition\": {\"latitude\": 11.0, \"longitude\": 21.0, \"altitude\": 20 }}"

echo "\nPrepare for usual landing!"
curl -i --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"Land\": null}"

echo "\nJourney"
curl -i --location $TEST_URL
