#!/bin/bash

# Check if the branch is 'main'
if [ "${GITHUB_REF#refs/heads/}" == "main" ]; then
  echo TWINE_PASSWORD="secrets.PYPI_API_TOKEN" >> $GITHUB_ENV
  echo "Release to PyPI"
else
  echo TWINE_PASSWORD="secrets.TEST_PYPI_API_TOKEN" >> $GITHUB_ENV
  echo "Release to Test PyPI"
fi
