#!/usr/bin/env bash

HOME="/root/finance"

cd "$HOME/finance-back/api"
git pull

BUILD_VERSION=$(git rev-parse HEAD)

echo "$(date --utc +%FT%TZ): Releasing new finance version : $BUILD_VERSION"

echo "$(date --utc +%FT%TZ): Running build..."
docker build -t finance .

echo "$(date --utc +%FT%TZ): Running container..."
cd /root
OLD_CONTAINER=$(docker ps -aqf "name=finance")

docker container rm -f $OLD_CONTAINER > /dev/null
docker compose up -d --no-deps --no-recreate finance
