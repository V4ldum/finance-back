#!/usr/bin/env bash

HOME="/home/finance"

cd "$HOME/finance-back/update-agent"
git pull

BUILD_VERSION=$(git rev-parse HEAD)

echo "$(date --utc +%FT%TZ): Releasing new finance update-agent version : $BUILD_VERSION"

echo "$(date --utc +%FT%TZ): Running build..."
docker build -o "$HOME/update-agent" .