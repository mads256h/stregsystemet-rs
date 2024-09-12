#!/bin/sh
set -e
set -u
set -x

cargo sqlx db create
cargo sqlx migrate run
