@echo off
set project_path=%cd%

docker run -t -i --name kphis -p 80:80 -p 443:443 --restart=always ^
-v %project_path%/volume:/volume ^
--init kphis public

:: USAGE:
::     backend [MODE]

:: ARGS:
::     <MODE>    Set config environment by config file name Ex. /volume/config/debug.toml -> debug
::               [default: debug]

:: OPTIONS:
::     -h, --help       Print help information
::     -V, --version    Print version information
