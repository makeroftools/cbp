#!/bin/bash
# remove_submodule.sh <path/to/submodule>

SUBMODULE=$1

if [ -z "$SUBMODULE" ]; then
  echo "Usage: $0 <path/to/submodule>"
  exit 1
fi

git submodule deinit -f -- "$SUBMODULE"
git rm -f "$SUBMODULE"
rm -rf ".git/modules/$SUBMODULE"
rm -rf "$SUBMODULE"

git commit -m "Remove submodule $SUBMODULE"
echo "Submodule $SUBMODULE removed cleanly."
