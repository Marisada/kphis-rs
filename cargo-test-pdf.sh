#!/bin/bash

# we use 1 thread for testing
if docker ps -a | grep -q test_maria ; then
    cargo test pdf -p kphis-api-pdf --lib --tests -- --ignored --nocapture --test-threads=1
else
    echo "please start test_maria docker and test again"
fi
