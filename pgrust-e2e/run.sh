#!/bin/bash
# Boot a pgrust binary (built with --features pgrx-ext) on a fresh C-initdb'd
# datadir and replay the pgrx example suite. Usage: run.sh [postgres-binary]
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$HERE/../../.." && pwd)"
BIN="${1:-$REPO/target/debug/postgres}"
PGBIN=/opt/homebrew/opt/postgresql@18/bin
export PGRUST_TZDIR="${PGRUST_TZDIR:-$("$PGBIN/pg_config" --sharedir)/timezone}"
export PGRUST_PGSHAREDIR="${PGRUST_PGSHAREDIR:-$("$PGBIN/pg_config" --sharedir)}"
ulimit -s 65520 2>/dev/null
WORK="${PGRX_E2E_WORK:-/tmp/pgrx-e2e-$$}"
PORT="${PGRX_E2E_PORT:-54329}"
rm -rf "$WORK"; mkdir -p "$WORK/sock"
"$PGBIN/initdb" -D "$WORK/dd" --no-locale --encoding=UTF8 -U postgres -A trust >"$WORK/initdb.log" 2>&1 || { echo initdb failed; exit 2; }
(RUST_MIN_STACK=67108864 RUST_BACKTRACE=1 exec "$BIN" -D "$WORK/dd" -k "$WORK/sock" -p "$PORT" \
    -c io_method=sync -c autovacuum=off >"$WORK/server.log" 2>&1) &
SRV=$!
for i in $(seq 1 120); do
    "$PGBIN/pg_isready" -h "$WORK/sock" -p "$PORT" -U postgres >/dev/null 2>&1 && break
    kill -0 $SRV 2>/dev/null || { echo "server died:"; tail -30 "$WORK/server.log"; exit 1; }
    sleep 0.5
done
"$PGBIN/psql" -h "$WORK/sock" -p "$PORT" -U postgres -X -a -q -d postgres -f "$HERE/pgrx-examples.sql" > "$WORK/out.txt" 2>&1
cat "$WORK/out.txt"
kill $SRV 2>/dev/null; wait $SRV 2>/dev/null
echo "--- server log tail ---"; tail -20 "$WORK/server.log"
echo "work dir: $WORK"
