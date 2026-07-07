# KPHIS `kphis-backend`

This binary implements the backend API to serving HTTP and HTTPs
endpoints for `kphis-frontend` sub-project by using `kphis-model` sub-project data structures.

## How to build
At project root
```bash
cargo clean
cargo update
cargo build --bin kphis-backend --release
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
    cargo build --bin kphis-backend --release
    ```

## Build on Windows with WSL to Linux executable
At project root
```bat
wsl $HOME/.cargo/bin/cargo build --bin kphis-backend --release
```

## How to run
1. From source
    ```bash
    cargo run --bin kphis-backend
    ```
2. From binary build
    - copy binary from `target/release/kphis-backend` to the same level as `/volume`
    - check `/volume/config/debug.toml` for database names and any settings you want
    - run with default config (`/volume/config/debug.toml`)
        ```bash
        kphis-backend
        ```
        or run with `/volume/config/XXXX.toml` config
        ```bash
        kphis-backend XXXX
        ```

---
This crate is part of the [KPHIS](https://github.com/Marisada/kphis) project.