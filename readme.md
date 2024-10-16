# stregsystem-rs

stregsystem-rs is the rust edition of the stregsystem, with additional improvements.
Some parts are still WIP.

## Getting started

Beware, this only works on Linux

- Clone the repo
- create .env file in root
- put `DATABASE_URL=postgres://stregsystemet:password@localhost/stregsystemet` in the .env
- then `cargo install sqlx-cli`
- run `./scripts/start_postgres.sh`
- run `./scripts/init_db.sh`
- then `cargo run`
  Congrats, you now have a running rust edition of the stregsystem.
