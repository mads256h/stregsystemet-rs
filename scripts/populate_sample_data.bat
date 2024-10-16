REM Equivalent to SCRIPTS="$(dirname "$0")"
set SCRIPTS=%~dp0

docker exec -i postgres-stregsystemet psql -f /dev/stdin "postgres://stregsystemet:password@localhost/stregsystemet" < "%SCRIPTS%sample_data.sql"
