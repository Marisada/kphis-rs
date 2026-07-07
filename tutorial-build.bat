@echo off

mdbook --version >nul 2>&1 && (
    mdbook build tutorial
) || (
    echo "please `cargo install mdbook` and try again"
)