#!/bin/bash

RANDOM=$$
TEST_ACCT="test-plane-$RANDOM"
TEST_URL="localhost:3030/plane/$TEST_ACCT"

echo "***************************"
echo "* Plane: $TEST_ACCT"
echo "***************************"

echo "Registering plane"
curl --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"Register\": {\"registration_id\": \"$TEST_ACCT\"}}"

echo -e "\nUpdating position"
curl --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"UpdatePosition\": {\"latitude\": 1.0, \"longitude\": 2.0, \"altitude\": 0 }}"

echo -e "\nPrepare for take off!"
curl --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"TakeOff\": null}"

echo -e "\nUpdating position"
curl --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"UpdatePosition\": {\"latitude\": 10.0, \"longitude\": 20.0, \"altitude\": 10 }}"

echo -e "\nJourney"
curl --location $TEST_URL

echo -e "\n\nUpdating position"
curl --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"UpdatePosition\": {\"latitude\": 11.0, \"longitude\": 21.0, \"altitude\": 20 }}"

echo -e "\nPrepare for usual landing!"
curl --location --request POST $TEST_URL --header 'Content-Type: application/json' --data-raw "{\"Land\": null}"

echo -e "\nJourney"
curl --location $TEST_URL
echo -e "\n"
