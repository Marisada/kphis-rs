@echo off
echo precompress files in `volume\pwa` and all subdirectories
precompress -c br:11,gz:9 volume\pwa
precompress -c br:11,gz:9 -e mjs,typ,webmanifest volume\pwa