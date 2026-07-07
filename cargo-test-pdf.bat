@echo off

docker --version >nul 2>&1 && (
    docker ps -a | findstr /c:"test_maria" && (
        goto run
     ) || (
        echo please start test_maria docker and try again
     )
) || (
    echo docker.exe not found..
    set /P in_wsl="Did you already run test_maria docker in Windows WSL ? <y|n>: "
    if "%in_wsl%"=="y" (
       goto run
    )
)

:: we use 1 thread for testing
:run
cargo test pdf -p kphis-api-pdf --lib --tests -- --ignored --nocapture --test-threads=1