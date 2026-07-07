# KPHIS `kphis-db-util`

This binary will update database Schema, Triggers and Stored Procedures.

## How to build
At project root
```bash
cargo clean
cargo update
cargo build --bin kphis-db-util --release
```

## Build on WSL in Windows to Linux executable
- Install Rust in WSL
- Clone from Windows
    ```bash
    git clone -o windows /mnt/c/Users/YOUR_NAME/GitHub/kphis
    ```
- Restore changes
    ```bash
    git restore .
    ```
- Pull `main` from Windows
    ```bash
    git pull windows
    ```
- Optional: pull `WORKING_BRANCH` from Windows
    ```bash
    git pull windows WORKING_BRANCH
    git switch WORKING_BRANCH
    ```
- Build at project root
    ```bash
    cargo build --bin kphis-db-util --release
    ```

## Build on Windows with WSL to Linux executable
At project root
```bat
wsl $HOME/.cargo/bin/cargo build --bin kphis-db-util --release
```

## How to run
1. From source
    ```bash
    cargo run --bin kphis-db-util
    ```
2. From binary build
    - copy binary from `target/release/kphis-db-util` to the same level as `/volume`
    - check `/volume/config/debug.toml` for database names and database settings
    - run update `Schema` (`-s`) with default config (`/volume/config/debug.toml`)
        ```bash
        kphis-db-util -u
        ```
        or update `Triggers` and `Stored Procedures` (`-t`) with `/volume/config/XXXX.toml` config file
        ```bash
        kphis-db-util -t XXXX
        ```
---
This crate is part of the [KPHIS](https://github.com/Marisada/kphis) project.