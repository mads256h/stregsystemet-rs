# stregsystem-rs

stregsystem-rs is the rust edition of the stregsystem, with additional improvements.
Some parts are still WIP.


## Building

Build the project with
```bash
cargo build
```


## Developing

Make sure that rust is installed on your system.
You will need to install `sqlx-cli`.
```bash
cargo install sqlx-cli
```

After this step create a file in the root of the project named `.env`.
This file should contain the following:
```bash
DATABASE_URL=postgres://stregsystemet:password@localhost/stregsystemet
```

The before you can build the project you need to run a postgres instance.
Included in the project are scripts that starts an emphemeral postgres instance using docker (or podman).
To start postgres run the following script:
```bash
./scripts/start_postgres.sh
```
On Windows this is
```cmd
scripts\start_postgres.bat
```

Afterwards the database schema must be initialized by running:
```bash
./scripts/init_db.sh
```
On Windows:
```cmd
scripts\init_db.bat
```

Optionally you can insert test data into the database:
```bash
./scripts/populate_sample_data.sh
```
On Windows:
```cmd
scripts\populate_sample_data.bat
```

You can now build and run the project with:
```bash
cargo run
```

Before you commit your changes you must format the code and satisfy the linter:
```bash
cargo fmt
cargo clippy
```

Lastly, you have to update the sqlx files if you have updated any files with sql:
```bash
cargo sqlx prepare
```
