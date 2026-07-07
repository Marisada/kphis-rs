#!/bin/bash

cargo test -p kphis-api-core
cargo test -p kphis-api-handler
cargo test -p kphis-api-pdf
cargo test -p kphis-api-query
cargo test -p kphis-api-router
cargo test -p kphis-backend
cargo test -p kphis-db-util
cargo test -p kphis-drg-worker
# cargo test -p kphis-frontend
cargo test -p kphis-model
cargo test -p kphis-sqlx-tester
cargo test -p kphis-typst-worker
cargo test -p kphis-ui-component
cargo test -p kphis-ui-core
# cargo test -p kphis-ui-page
cargo test -p kphis-util
cargo test -p kphis-worker
# cargo test -p wasm-tests