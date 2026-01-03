#!/bin/bash
# add_submodule.sh <repo-url> <path>
#
# Make executable: chmod +x add_submodule.sh
# Run: ./add_submodule.sh https://github.com/user/repo.git runtimes/nodejs/project

REPO=$1
PATH=$2

if [ -z "$REPO" ] || [ -z "$PATH" ]; then
  echo "Usage: $0 <repo-url> <path>"
  exit 1
fi

git submodule add "$REPO" "$PATH"
git submodule update --init --recursive "$PATH"
git commit -m "Add submodule $PATH from $REPO"

echo "Submodule $PATH added and initialized."
