#!/bin/sh
set -e
set -u
set -x

SCRIPTS="$(dirname "$0")"

docker exec -i postgres-stregsystemet psql -f /dev/stdin "postgres://stregsystemet:password@localhost/stregsystemet" < "$SCRIPTS/sample_data.sql"
