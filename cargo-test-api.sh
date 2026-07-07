#!/bin/bash

# we use 1 thread for testing
if docker ps -a | grep -q test_maria ; then
    cargo test api -p kphis-api-router --lib --tests -- --ignored --test-threads=1
else
    echo "please start test_maria docker and test again"
fi
