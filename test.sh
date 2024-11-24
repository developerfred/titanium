#!/bin/bash


docker-compose --profile test run test


TEST_EXIT_CODE=$?


docker-compose --profile test down


exit $TEST_EXIT_CODE