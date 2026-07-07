# Integration tests

## WASM
We collected all `WASM` tests here to prevent dev-dependencies conflicts (from [getrandom](https://docs.rs/getrandom/latest/getrandom/#webassembly-support)).

- Using headless `Edge`, `Chrome` or `Firefox` 
    * Install FireFox geckodriver via cargo
        ```bash
        cargo install geckodriver
        ```
        then run test command
        * Windows
            ```bat
            cargo-test-wasm
            ```
        * Linux
            ```bash
            ./cargo-test-wasm.sh
            ```
    * Using web-driver binary
        1. Download web-driver
            * geckodriver - https://github.com/mozilla/geckodriver/releases
            * chromedriver - https://chromedriver.chromium.org/downloads
            * msedgedriver - https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/
            * safaridriver - should be preinstalled on OSX    
        2. Set ENV
            * Windows
            ```bat
            set GECKODRIVER=E:\path\to\geckodriver.exe
            set CHROMEDRIVER=E:\path\to\chromedriver.exe
            set MSEDGEDRIVER=E:\path\to\msedgedriver.exe
            ```
            * Linux
            ```bash
            GECKODRIVER="/path/to/geckodriver"
            CHROMEDRIVER="/path/to/chromedriver"
            MSEDGEDRIVER="/path/to/msedgedriver"
            ```
        3. Run all tests
        ```bash
        cargo test --target wasm32-unknown-unknown -p wasm-tests
    ```

- Run on browser
    1. Set ENV
        * Windows
        ```bat
        set NO_HEADLESS=1
        ```
        * Linux
        ```bash
        NO_HEADLESS=1
        ```
    2. Run only 1 test each time (all is `wasm_datetime`, `wasm_route`, `wasm_token`)
    ```bash
    cargo test --target wasm32-unknown-unknown -p wasm-tests -test wasm_datetime
    cargo test --target wasm32-unknown-unknown -p wasm-tests -test wasm_route
    cargo test --target wasm32-unknown-unknown -p wasm-tests -test wasm_token
    ```
    3. Open web browser at `127.0.0.1:8000` for each test