#!/bin/bash/env bash

git log | grep git-subtree-dir | tr -d ' ' | cut -d ":" -f2 | sort | uniq