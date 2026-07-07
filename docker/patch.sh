#!/bin/bash

set -e

echo "Backup local files.."
cp -r kphis/volume/pwa/local kphis-volume-pwa-local >/dev/null

echo "Update files.."
tar -xzf kphis.tar.gz -C .
chown -R dockeruser:dockergrp docker
rm -f kphis/kphis.tar
mv docker/kphis.tar kphis/
rm -f kphis/kphis-db-util
mv docker/kphis-db-util kphis/
chmod +x kphis/kphis-db-util
rm -R kphis/volume/pwa
mv docker/volume/pwa kphis/volume/
rm -f kphis/volume/app_assets.bin

echo "Copy local files.."
mv kphis-volume-pwa-local kphis/volume/pwa/local

cd kphis
echo -n "Patch Database with public.toml.. "
./kphis-db-util -st public
echo -n "Stop docker container.. "
docker stop kphis
echo -n "Remove docker container.. "
docker rm kphis
echo -n "Remove docker image.. "
docker image rm kphis
docker load -i kphis.tar
echo -n "Run.. "
./run.sh

cd ..

echo
echo "Please check with 'docker logs kphis'"
echo "If OK, please remove 'docker' folder and 'kphis.tar.gz'"