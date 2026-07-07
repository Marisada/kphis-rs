@echo off
if "%~1"=="client" (
    set name=client
    set crate_name=kphis-frontend
    set target_name=kphis_frontend
    goto set_mode
)
if "%~1"=="typst" (
    set name=typst_worker
    set crate_name=kphis-typst-worker
    set target_name=kphis_typst_worker
    goto set_mode
)
if "%~1"=="drg" (
    set name=drg_worker
    set crate_name=kphis-drg-worker
    set target_name=kphis_drg_worker
    goto set_mode
) else (
    goto usage
)

:set_mode
if "%~2"=="" goto dev
if "%~2"=="dev" goto dev
if "%~2"=="release" (
    goto release
) else (
    goto usage
)

:release
set mode=--release
set mode_name=release
set br=11
set gz=9
goto set_precomp

:dev
set mode=
set mode_name=debug
set br=3
set gz=3
goto set_precomp

:set_precomp
if "%~3"=="" goto raw
if "%~3"=="raw" goto raw
if "%~3"=="precompress" (
    goto precompress
) else (
    goto usage
)

:precompress
set precompress=1
goto run

:raw
set precompress=0
goto run

:run
if defined name (
	echo creating %name% WASM in %mode_name% mode
) else (
	goto usage
)
set pwa_path=volume\pwa
set project_path=%cd%

cd %pwa_path%

if exist "%name%_bg.wasm" (
    del %name%_bg.wasm
)
if exist "%name%_bg.wasm.br" (
    del %name%_bg.wasm.br
)
if exist "%name%_bg.wasm.gz" (
    del %name%_bg.wasm.gz
)

if exist "%name%.js" (
    del %name%.js
)
if exist "%name%.js.br" (
    del %name%.js.br
)
if exist "%name%.js.gz" (
    del %name%.js.gz
)

if exist "sw.js" (
    del sw.js
)
if exist "sw.js.br" (
    del sw.js.br
)
if exist "sw.js.gz" (
    del sw.js.gz
)

if "%~1"=="client" (
    if exist "snippets" (
        rmdir /S /Q snippets
    )
)

cd %project_path%

:: wasm-pack build --target web --out-name %name% --out-dir wasm-pack/ --%mode%
cargo build --package %crate_name% --target wasm32-unknown-unknown %mode%
if %errorlevel% neq 0 (
    exit /b %errorlevel%
)

move target\wasm32-unknown-unknown\%mode_name%\%target_name%.wasm target\wasm32-unknown-unknown\%mode_name%\%name%.wasm
wasm-bindgen --target web --out-dir wasm-pack\ target\wasm32-unknown-unknown\%mode_name%\%name%.wasm
if %errorlevel% neq 0 (
    exit /b %errorlevel%
)

if "%mode_name%"=="release" (
    wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int --enable-simd wasm-pack\%name%_bg.wasm -o wasm-pack\%name%_bg.wasm
    if %errorlevel% neq 0 (
        exit /b %errorlevel%
    )
)

move wasm-pack\%name%_bg.wasm %project_path%\%pwa_path%\
move wasm-pack\%name%.js %project_path%\%pwa_path%\
if "%~1"=="client" (
    :: move /Y wasm-pack\snippets %project_path%\%pwa_path%\
    xcopy wasm-pack\snippets %project_path%\%pwa_path%\snippets /E /H /C /I >NUL
)

cd %project_path%\%pwa_path%

if exist "%name%_bg.wasm" (
    echo build %project_path%\%pwa_path%\%name%_bg.wasm successfully
)
if exist "%name%.js" (
    echo build %project_path%\%pwa_path%\%name%.js successfully
)
if "%~1"=="client" (
    if exist "snippets" (
        echo build snippets successfully
    )
)

For /f "tokens=2-4 delims=/ " %%a in ('date /t') do (set date_now=%%c%%a%%b)
For /f "tokens=1-3 delims=/:." %%a in ("%TIME: =0%") do (set time_now=%%a%%b%%c)

echo const VERSION = '%date_now%-%time_now%' > sw.js
type sw_template.js >> sw.js

echo update sw.js version to %date_now%-%time_now%

cd %project_path%

if "%precompress%"=="1" (
    echo precompress %project_path%\%pwa_path%\%name%_bg.wasm
    precompress -c br:%br%,gz:%gz% %project_path%\%pwa_path%\%name%_bg.wasm
    echo precompress %project_path%\%pwa_path%\%name%.js
    precompress -c br:%br%,gz:%gz% %project_path%\%pwa_path%\%name%.js
    echo precompress %project_path%\%pwa_path%\sw.js
    precompress -c br:%br%,gz:%gz% %project_path%\%pwa_path%\sw.js
)
echo done.

goto end

:usage
echo Usage: %0 name mode
echo   - name: WASM name ('client', 'typst' or 'drg')
echo   - mode: compile mode ('release' or 'dev', default = 'dev')
echo   - precompress: precompress mode ('precompress' or 'raw', default = 'raw')
echo ex: %0 client release precompress
echo ex: %0 typst
exit /b 1

:end
