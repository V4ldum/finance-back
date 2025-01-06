#!/usr/bin/env bash


LOCK_FILE="/tmp/finance.lock"
HOME="/home/finance"
flock -n $LOCK_FILE "bash $HOME/finance-back/api/scripts/deploy_if_changed.sh" >> "$HOME/logs/finance-deploy.log" 2>&1