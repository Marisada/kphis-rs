#!/bin/sh
DIR=$(pwd)

docker run -i -t -p 80:80 -p 443:443 --rm \
    -v ${DIR}/volume:/volume \
    --init kphis debug

# USAGE:
#     backend [MODE]

# ARGS:
#     <MODE>    Set config environment by config file name Ex. /volume/config/debug.toml -> debug
#               [default: debug]

# OPTIONS:
#     -h, --help       Print help information
#     -V, --version    Print version information
