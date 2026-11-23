@echo off
set start_time=%time%
set pwa_path=volume\pwa
set css_path=statics\css

:: for color echo
set gold=[93m
set nc=[0m

:: clear
rmdir /S /Q docker\volume
rmdir /S /Q docker\volume-pwa-local
del docker\kphis.tar >NUL
del docker\kphis.tar.gz >NUL
del docker\kphis-db-util >NUL
del docker\kphis-db-util.exe >NUL

:: install tools
echo: & echo.%gold%Install tools..%nc%
cargo install wasm-opt
cargo install wasm-bindgen-cli
cargo install grass
cargo install mdbook
cargo install precompress

:: prepared files
echo: & echo.%gold%Build Drg WASM..%nc%
call wasm.bat drg release

echo: & echo.%gold%Build Typst WASM..%nc%
call wasm.bat typst release

echo: & echo.%gold%Build Client WASM..%nc%
call wasm.bat client release

echo: & echo.%gold%Build kphis-db-util..%nc%
wsl $HOME/.cargo/bin/cargo build --bin kphis-db-util --release
move target\release\kphis-db-util docker\

echo: & echo.%gold%Build CSS..%nc%
grass -s compressed sass\src\app.scss %pwa_path%\app.min.css
grass -s compressed sass\src\bootstrap\scss\bootstrap.scss %pwa_path%\%css_path%\bootstrap.min.css
grass -s compressed sass\src\font-awesome\scss\fontawesome.scss %pwa_path%\%css_path%\font-awesome.min.css

echo: & echo.%gold%Build tutorial pages..%nc%
rmdir /S /Q tutorial\kphis-book
rmdir /S /Q tutorial\src
rmdir /S /Q tutorial\theme
git clone https://github.com/Marisada/kphis-book tutorial\kphis-book
xcopy tutorial\kphis-book\src tutorial\src /E /H /C /I >NUL
xcopy tutorial\kphis-book\theme tutorial\theme /E /H /C /I >NUL
mdbook build tutorial

echo: & echo.%gold%Precompress static files..%nc%
precompress -c br:11,gz:9 %pwa_path%
precompress -c br:11,gz:9 -e mjs,typ,webmanifest %pwa_path%

:: copy associated files
echo: & echo.%gold%Copy associated files..%nc%
if exist "volume\cert" (
    echo copy volume\cert to docker directory
    xcopy volume\cert docker\volume\cert /E /H /C /I >NUL
)
echo copy volume\config to docker directory
xcopy volume\config docker\volume\config /E /H /C /I >NUL
echo copy %pwa_path% to docker directory
xcopy %pwa_path% docker\volume\pwa /E /H /C /I >NUL
mkdir docker\volume\logs
mkdir docker\volume\images
mkdir docker\volume\thumbs
xcopy volume-pwa-local docker\volume-pwa-local /E /H /C /I >NUL

:: docker build
echo: & echo.%gold%Build docker image..%nc%
wsl docker build -t kphis .
if %errorlevel% neq 0 exit /b %errorlevel%

:: docker save
echo: & echo.%gold%Save docker image..%nc%
wsl docker save kphis > docker\kphis.tar
if %errorlevel% neq 0 exit /b %errorlevel%

:: create tar ball
echo: & echo.%gold%Create tar ball..%nc%
tar -czf kphis.tar.gz docker
move kphis.tar.gz docker\

:: clear
rmdir /S /Q docker\volume
del docker\kphis.tar
del docker\kphis-db-util

if exist "docker\kphis.tar.gz" (
    echo: & echo.%gold%docker/kphis.tar.gz%nc% created.
)

echo Start Time: %start_time%
echo Finish Time: %time%