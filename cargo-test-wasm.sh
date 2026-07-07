#!/bin/bash

# MUST download web-driver from
#    * geckodriver - https://github.com/mozilla/geckodriver/releases or 'cargo install geckodriver'
#    * chromedriver - https://chromedriver.chromium.org/downloads
#    * msedgedriver - https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/
#    * safaridriver - should be preinstalled on OSX

# Clean environment variables
unset NO_HEADLESS
unset GECKODRIVER
unset CHROMEDRIVER
unset MSEDGEDRIVER

# Set web-driver variable
# by remove comment(::) on web-driver you want to test

# GECKODRIVER="/home/user/geckodriver"
# CHROMEDRIVER="/home/user/chromedriver"
# MSEDGEDRIVER="/home/user/msedgedriver"
cargo test --target wasm32-unknown-unknown -p wasm-tests