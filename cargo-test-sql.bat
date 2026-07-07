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

:: we use 1 thread for testing CRUD
:run
cargo test sqlx -p kphis-sqlx-tester --lib -- --ignored --test-threads=1
cargo test sqlx -p kphis-api-core -p kphis-api-query -p kphis-api-pdf --lib -- --ignored --test-threads=1