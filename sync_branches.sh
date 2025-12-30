#!/bin/bash
BRANCH=${1:-main}  # Default 'main'
git submodule foreach git checkout $BRANCH
git submodule foreach git pull origin $BRANCH
git add .
git commit -m "Sync submodules to $BRANCH"
echo "Submodules synced to $BRANCH."
