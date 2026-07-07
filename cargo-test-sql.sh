#!/bin/bash

# we use 1 thread for testing CRUD
if docker ps -a | grep -q test_maria ; then
    cargo test sqlx -p kphis-sqlx-tester --lib -- --ignored --test-threads=1
    cargo test sqlx -p kphis-api-core -p kphis-api-query -p kphis-api-pdf --lib -- --ignored --test-threads=1
else
    echo "please start test_maria docker and test again"
fi
