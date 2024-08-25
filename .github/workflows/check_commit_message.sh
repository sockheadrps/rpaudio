#!/bin/bash

# Fetch the commit message from the latest commit
COMMIT_MESSAGE=$(git log -1 --pretty=%B)

# Check if the commit message starts with "BUILD"
if [[ $COMMIT_MESSAGE == BUILD* ]]; then
  echo "BUILD condition met."
  exit 0
else
  echo "BUILD condition not met."
  exit 1
fi