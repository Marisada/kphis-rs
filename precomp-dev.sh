#!/bin/bash

set -e

echo "fastest precompress files in 'volume/pwa' and all subdirectories"
precompress -c br:0,gz:0 volume/pwa
precompress -c br:0,gz:0 -e mjs,typ,webmanifest volume/pwa