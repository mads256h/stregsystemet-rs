@echo off
setlocal enabledelayedexpansion

REM Exit immediately if a command exits with a non-zero status (set -e equivalent)
if not defined ERRORLEVEL goto :error

REM Print each command before executing it (set -x equivalent)
@echo on

REM Get the directory of the current script (equivalent to SCRIPTS="$(dirname "$0")")
set SCRIPTS=%~dp0

REM Run the docker exec command
docker exec -i postgres-stregsystemet psql -f /dev/stdin "postgres://stregsystemet:password@localhost/stregsystemet" < "%SCRIPTS%sample_data.sql"
if %errorlevel% neq 0 goto :error

goto :eof

:error
echo "An error occurred."
exit /b 1
