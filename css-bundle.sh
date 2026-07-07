#!/bin/bash

set -e

PWA_PATH="volume/pwa"
CSS_PATH="statics/css"

echo "create ${PWA_PATH}/app.min.css"
grass -s compressed sass/src/app.scss ${PWA_PATH}/app.min.css

echo "create ${PWA_PATH}/${CSS_PATH}/bootstrap.min.css"
grass -s compressed sass/src/bootstrap/scss/bootstrap.scss ${PWA_PATH}/${CSS_PATH}/bootstrap.min.css

echo "create ${PWA_PATH}/${CSS_PATH}/font-awesome.min.css"
grass -s compressed sass/src/font-awesome/scss/fontawesome.scss ${PWA_PATH}/${CSS_PATH}/font-awesome.min.css

echo "create ${PWA_PATH}/${CSS_PATH}/font-awesome-solid.min.css"
grass -s compressed sass/src/font-awesome/scss/solid.scss ${PWA_PATH}/${CSS_PATH}/font-awesome-solid.min.css

echo "create ${PWA_PATH}/${CSS_PATH}/font-awesome-regular.min.css"
grass -s compressed sass/src/font-awesome/scss/regular.scss ${PWA_PATH}/${CSS_PATH}/font-awesome-regular.min.css

echo "precompress ${PWA_PATH}/app.min.css"
precompress -c br:11,gz:9 ${PWA_PATH}/app.min.css

echo "precompress ${PWA_PATH}/${CSS_PATH}/bootstrap.min.css"
precompress -c br:11,gz:9 ${PWA_PATH}/${CSS_PATH}/bootstrap.min.css

echo "precompress ${PWA_PATH}/${CSS_PATH}/font-awesome.min.css"
precompress -c br:11,gz:9 ${PWA_PATH}/${CSS_PATH}/font-awesome.min.css

echo -e "\\n"
./reload.sh
