#!/bin/bash

set -e

function cleaning {
    rm -rf docker/volume
    rm -rf docker/volume-pwa-local
    rm -f docker/kphis.tar
    rm -f docker/kphis-db-util
}

PWA_PATH="volume/pwa"
CSS_PATH="statics/css"

# for color echo
GOLD='\033[0;93m'
NC='\033[0m' # No Color

# clear
cleaning
rm -f docker/kphis.tar.gz

# install tools
echo -e "\\n${GOLD}Install tools..${NC}"
cargo install wasm-opt
cargo install wasm-bindgen-cli
cargo install grass
cargo install mdbook
cargo install precompress

# prepared files
echo -e "\\n${GOLD}Build Drg WASM..${NC}"
if [ -e "${PWA_PATH}/drg_worker_bg.wasm" ]; then
    if read -r -p "Do you want to skip? (y/n)" -n 1 -t 5; then
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]
        then
            echo "Skipped"
        else
            ./wasm.sh "drg" "release"
        fi
    else
        echo
        ./wasm.sh "drg" "release"
    fi
else
    ./wasm.sh "drg" "release"
fi

echo -e "\\n${GOLD}Build Typst WASM..${NC}"
if [ -e "${PWA_PATH}/typst_worker_bg.wasm" ]; then
    if read -r -p "Do you want to skip? (y/n)" -n 1 -t 5; then
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]
        then
            echo "Skipped"
        else
            ./wasm.sh "typst" "release"
        fi
    else
        echo
        ./wasm.sh "typst" "release"
    fi
else
    ./wasm.sh "typst" "release"
fi

echo -e "\\n${GOLD}Build Client WASM..${NC}"
if [ -e "${PWA_PATH}/client_bg.wasm" ]; then
    if read -r -p "Do you want to skip? (y/n)" -n 1 -t 5; then
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]
        then
            echo "Skipped"
        else
            ./wasm.sh "client" "release"
        fi
    else
        echo
        ./wasm.sh "client" "release"
    fi
else
    ./wasm.sh "client" "release"
fi

echo -e "\\n${GOLD}Build kphis-db-util..${NC}"
cargo build --bin kphis-db-util --release
cp target/release/kphis-db-util docker/kphis-db-util

echo -e "\\n${GOLD}Build CSS..${NC}"
grass -s compressed sass/src/app.scss ${PWA_PATH}/app.min.css
grass -s compressed sass/src/bootstrap/scss/bootstrap.scss ${PWA_PATH}/${CSS_PATH}/bootstrap.min.css
grass -s compressed sass/src/font-awesome/scss/fontawesome.scss ${PWA_PATH}/${CSS_PATH}/font-awesome.min.css

echo -e "\\n${GOLD}Build tutorial pages..${NC}"
rm -rf tutorial/kphis-book
rm -rf tutorial/src
rm -rf tutorial/theme
git clone https://github.com/Marisada/kphis-book tutorial/kphis-book
cp -r tutorial/kphis-book/src tutorial/src >/dev/null
cp -r tutorial/kphis-book/theme tutorial/theme >/dev/null
mdbook build tutorial

echo -e "\\n${GOLD}Precompress static files..${NC}"
precompress -c br:11,gz:9 ${PWA_PATH}
precompress -c br:11,gz:9 -e mjs,typ,webmanifest ${PWA_PATH}

# copy associated files
echo -e "\\n${GOLD}Copy associated files..${NC}"
mkdir -p docker/volume/cert
if [ -e "volume/cert" ]; then
    echo -e "copy volume/cert to docker directory"
    cp -r volume/cert docker/volume/ >/dev/null
fi
echo -e "copy volume/config to docker directory"
mkdir -p docker/volume/config && cp -r volume/config docker/volume/ >/dev/null
echo -e "copy ${PWA_PATH} to docker directory"
mkdir -p docker/volume/pwa && cp -r ${PWA_PATH} docker/volume/ >/dev/null
mkdir -p docker/volume/logs
mkdir -p docker/volume/images
mkdir -p docker/volume/thumbs
mkdir -p docker/volume-pwa-local && cp -r volume-pwa-local docker/ >/dev/null

# docker build
echo -e "\\n${GOLD}Build docker image..${NC}"
docker build -t kphis .

# docker save
echo -e "\\n${GOLD}Save docker image..${NC}"
docker save kphis > docker/kphis.tar

# create tar ball
echo -e "\\n${GOLD}Create tar ball..${NC}"
tar -czf kphis.tar.gz docker
mv kphis.tar.gz docker/

# clear
cleaning

if [ -e "docker/kphis.tar.gz" ]; then
    echo -e "\\n${GOLD}docker/kphis.tar.gz${NC} created."
fi

duration=$SECONDS
echo "$((duration / 60)) minutes and $((duration % 60)) seconds elapsed."