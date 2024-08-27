#!/bin/bash

# Check if the branch is 'main'
if [ "${GITHUB_REF#refs/heads/}" == "main" ]; then
  echo TWINE_PASSWORD="secrets.PYPI_API_TOKEN" >> $GITHUB_ENV
else
  echo TWINE_PASSWORD=secrets.TEST_PYPI_API_TOKEN" >> $GITHUB_ENV
fi
