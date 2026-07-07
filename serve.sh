#!/bin/bash

if [ -e "volume/app_assets.bin" ]; then
    rm volume/app_assets.bin
fi

cargo run --bin kphis-backend
