#!/bin/sh

# Exit immediately if any command fails
set -e

# Sets the environment variable, which specifies the location for pip to store its cache files
export XDG_CACHE_HOME=./my_player/.pip_cache

# Sets the environment variable, which adds the directory to the list of paths that Python searches for modules and packages when they are imported.
export PYTHONPATH=./my_player/packages:$PYTHONPATH

# Install the socha package
pip install --no-index --find-links=./my_player/dependencies/ ./my_player/dependencies/socha-1.0.1-py3-none-any.whl ./my_player/dependencies/xsdata-22.7-py3-none-any.whl --target=./my_player/packages/ --cache-dir=./my_player/.pip_cache

# Run the logic.py script with start arguments
python3 ./my_player/logic.py "$@"
