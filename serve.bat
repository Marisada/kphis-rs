@echo off

if exist "volume\app_assets.bin" (
    del volume\app_assets.bin
)

cargo run --bin kphis-backend
