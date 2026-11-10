#!/usr/bin/env bash
#LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#
# Run a command, and run it again if it fails: three runs in all, 10 and then 20 seconds apart.
# This is for downloaders such as rustup and apt-get, whose exit status does not tell a dropped
# connection from any other error, so every failure is retried.

set -euo pipefail

attempt=1
until "$@"; do
    status=$?
    if (( attempt == 3 )); then
        exit "$status"
    fi
    echo "retrying \`$*\` after attempt $attempt failed with exit status $status" >&2
    sleep $((attempt * 10))
    attempt=$((attempt + 1))
done
