#!/bin/bash
git submodule update --init --recursive --remote
git submodule foreach git pull origin main  # Assumes 'main' branch; adjust as needed
git add .
git commit -m "Update submodules to latest"
echo "Submodules updated."