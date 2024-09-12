#!/bin/sh
set -e
set -u
set -x

docker run                         \
  -d                               \
  --name 'postgres-stregsystemet'  \
  -l 'postgres-stregsystemet'      \
  -e POSTGRES_USER='stregsystemet' \
  -e POSTGRES_PASSWORD='password'  \
  -e POSTGRES_DB='stregsystemet'   \
  -p 5432:5432                     \
  docker.io/postgres:16