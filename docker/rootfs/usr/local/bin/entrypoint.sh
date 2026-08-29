#!/bin/sh
# shellcheck shell=sh
# entrypoint.sh — container startup script
# Alpine runtime images ship /bin/sh (busybox ash) but not bash by default.
# Called by: tini -> entrypoint.sh -> app
# Add pre-start setup here (env checks, dir creation, etc.)
exec "$@"
