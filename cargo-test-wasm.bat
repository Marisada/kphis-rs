@echo off

:: MUST download web-driver from
::    * geckodriver - https://github.com/mozilla/geckodriver/releases or 'cargo install geckodriver'
::    * chromedriver - https://chromedriver.chromium.org/downloads
::    * msedgedriver - https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/
::    * safaridriver - should be preinstalled on OSX

:: Clean environment variables
set NO_HEADLESS=
set GECKODRIVER=
set CHROMEDRIVER=
set MSEDGEDRIVER=

:: Set web-driver variable
:: by remove comment(::) on web-driver you want to test
:: NOTE: Do NOT set GECKODRIVER if you installed geckodriver via 'cargo install geckodriver' command

:: set GECKODRIVER=D:\webdrivers\geckodriver.exe
:: set CHROMEDRIVER=D:\webdrivers\chromedriver.exe
:: set MSEDGEDRIVER=D:\webdrivers\msedgedriver.exe

cargo test --target wasm32-unknown-unknown -p wasm-tests
