#!/bin/sh

cargo sqlx db create
cargo sqlx migrate run
