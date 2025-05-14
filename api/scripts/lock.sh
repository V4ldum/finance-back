#!/usr/bin/env bash


LOCK_FILE="/tmp/build.lock"
HOME="/root/finance"
flock -n $LOCK_FILE -c "bash $HOME/finance-back/api/scripts/deploy_if_changed.sh" >> "$HOME/logs/finance-deploy.log" 2>&1
