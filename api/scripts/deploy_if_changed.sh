#!/usr/bin/env bash

HOME="/home/finance"
cd "$HOME/finance-back"

#echo "$(date --utc +%FT%TZ): Fetching remote repository..."
git fetch
DIFF_API=$(git diff main origin/main --name-only | grep -v .idea | grep -e "^api/" > /dev/null; echo $?)
DIFF_UPDATE_AGENT=$(git diff main origin/main --name-only | grep -v .idea | grep -e "^update-agent/" > /dev/null; echo $?)

UPSTREAM=${1:-'@{u}'}
LOCAL=$(git rev-parse @)
REMOTE=$(git rev-parse "$UPSTREAM")
BASE=$(git merge-base @ "$UPSTREAM")

if [ $LOCAL = $REMOTE ]; then
        #echo "$(date --utc +%FT%TZ): No changes detected"
        :
elif [ $LOCAL = $BASE ]; then
        BUILD_VERSION=$(git rev-parse HEAD)

        if [ $DIFF_API = 0 ]; then
                echo "$(date --utc +%FT%TZ): Changes detected in api/, deploying new version: $BUILD_VERSION"
                bash api/scripts/deploy.sh
        fi

        if [ $DIFF_UPDATE_AGENT = 0 ]; then
                echo "$(date -utc +%FT%TZ): Changes detected in update-agent/, deploying new version: $BUILD_VERSION"
                bash update-agent/scripts/deploy.sh
        fi
elif [ $REMOTE = $BASE ]; then
        BUILD_VERSION=$(git rev-parse HEAD)

        echo "$(date --utc +%FT%TZ): Local changes detected, stashing"
        git stash

        if [ $DIFF_API = 0 ]; then
                echo "$(date -utc +%FT%TZ): Changes detected in api/, deploying new version: $BUILD_VERSION"
                bash api/scripts/deploy.sh
        fi

        if [ $DIFF_UPDATE_AGENT = 0 ]; then
                echo "$(date -utc +%FT%TZ): Changes detected in update-agent/, deploying new version: $BUILD_VERSION"
                bash update-agent/scripts/deploy-update.sh
        fi
else
        echo "$(date --utc +%FT%TZ): Git is diverged, this is unexpected."
fi