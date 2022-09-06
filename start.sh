#!/bin/sh
chmod +x logic.py
. venv/bin/activate
python ./logic.py "$@"
