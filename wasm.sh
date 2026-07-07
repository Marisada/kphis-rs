#!/bin/bash

set -e

function usage {
    echo "Usage: $0 name mode precompress"
    echo "  - name: WASM name ('client', 'typst' or 'drg')"
    echo "  - mode: compile mode ('release' or 'dev', default = 'dev')"
    echo "  - precompress: precompress mode ('precompress' or 'raw', default = 'raw')"
    echo "ex: $0 client release precompress"
    echo "ex: $0 typst"
    exit 1
}

if [ "$1" == "client" ]; then
    NAME="client"
    CRATE_NAME="kphis-frontend"
    TARGET_NAME="kphis_frontend"
elif [ "$1" == "typst" ]; then
    NAME="typst_worker"
    CRATE_NAME="kphis-typst-worker"
    TARGET_NAME="kphis_typst_worker"
elif [ "$1" == "drg" ]; then
    NAME="drg_worker"
    CRATE_NAME="kphis-drg-worker"
    TARGET_NAME="kphis_drg_worker"
else
    usage
fi

if [ "$2" == "release" ]; then
    MODE="--release"
    MODE_NAME="release"
    BR=11
    GZ=9
elif [ "$2" == "dev" ]; then
    MODE=""
    MODE_NAME="debug"
    BR=3
    GZ=3
elif [ "$2" == "" ]; then
    MODE=""
    MODE_NAME="debug"
    BR=3
    GZ=3
else
    usage
fi

if [ "$3" == "precompress" ]; then
    PRECOMP=1
elif [ "$3" == "raw" ]; then
    PRECOMP=0
elif [ "$3" == "" ]; then
    PRECOMP=0
else
    usage
fi

echo "creating ${NAME} WASM in ${MODE_NAME} mode $3"

PWA_PATH="volume/pwa"
PROJECT_PATH=$(pwd)

cd $PWA_PATH

if [ -e "${NAME}_bg.wasm" ]; then
    rm ${NAME}_bg.wasm
fi
if [ -e "${NAME}_bg.wasm.br" ]; then
    rm ${NAME}_bg.wasm.br
fi
if [ -e "${NAME}_bg.wasm.gz" ]; then
    rm ${NAME}_bg.wasm.gz
fi

if [ -e "${NAME}.js" ]; then
    rm ${NAME}.js
fi
if [ -e "${NAME}.js.br" ]; then
    rm ${NAME}.js.br
fi
if [ -e "${NAME}.js.gz" ]; then
    rm ${NAME}.js.gz
fi

if [ -e "sw.js" ]; then
    rm sw.js
fi
if [ -e "sw.js.br" ]; then
    rm sw.js.br
fi
if [ -e "sw.js.gz" ]; then
    rm sw.js.gz
fi

if [ "$1" == "client" ]; then
    if [ -e "snippets" ]; then
        rm -rf snippets
    fi
fi

cd ${PROJECT_PATH}

# if ($( wasm-pack build --target web --out-name $NAME --out-dir wasm-pack/ --$MODE )) ; then
if ($( cargo build --package ${CRATE_NAME} --target wasm32-unknown-unknown ${MODE} )) ; then
    rm -f target/wasm32-unknown-unknown/${MODE_NAME}/${NAME}.wasm
    mv -f target/wasm32-unknown-unknown/${MODE_NAME}/${TARGET_NAME}.wasm target/wasm32-unknown-unknown/${MODE_NAME}/${NAME}.wasm
    if ($( wasm-bindgen --target web --out-dir wasm-pack/ target/wasm32-unknown-unknown/${MODE_NAME}/${NAME}.wasm )) ; then
        if [ "${MODE_NAME}" == "release" ]; then
            if ($( wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int --enable-simd wasm-pack/${NAME}_bg.wasm -o wasm-pack/${NAME}_bg.wasm )) ; then
                echo
            else
                exit 1
            fi
        fi
        mv -f wasm-pack/${NAME}_bg.wasm ${PROJECT_PATH}/${PWA_PATH}/
        mv -f wasm-pack/${NAME}.js ${PROJECT_PATH}/${PWA_PATH}/
        if [ "$1" == "client" ]; then
            mv -f wasm-pack/snippets ${PROJECT_PATH}/${PWA_PATH}/
        fi
    else
        exit 1
    fi
else
    exit 1
fi

cd ${PROJECT_PATH}/${PWA_PATH}

if [ -e "${NAME}_bg.wasm" ]; then
    echo "build ${PROJECT_PATH}/${PWA_PATH}/${NAME}_bg.wasm successfully"
fi
if [ -e "${NAME}.js" ]; then
    echo "build ${PROJECT_PATH}/${PWA_PATH}/${NAME}.js successfully"
fi
if [ "$1" == "client" ]; then
    if [ -e "snippets" ]; then
        echo "build snippets successfully"
    fi
fi

VERSION=$(date '+%Y%m%d-%H%M%S')
echo "const VERSION = '${VERSION}'" > sw.js
cat sw_template.js >> sw.js

echo "update sw.js version to ${VERSION}"

cd ${PROJECT_PATH}

if [ "$PRECOMP" == 1 ]; then
    echo "precompress ${PROJECT_PATH}/${PWA_PATH}/${NAME}_bg.wasm"
    precompress -c br:${BR},gz:${GZ} ${PROJECT_PATH}/${PWA_PATH}/${NAME}_bg.wasm
    echo "precompress ${PROJECT_PATH}/${PWA_PATH}/${NAME}.js"
    precompress -c br:${BR},gz:${GZ} ${PROJECT_PATH}/${PWA_PATH}/${NAME}.js
    echo "precompress ${PROJECT_PATH}/${PWA_PATH}/sw.js"
    precompress -c br:${BR},gz:${GZ} ${PROJECT_PATH}/${PWA_PATH}/sw.js
fi

echo "done."
