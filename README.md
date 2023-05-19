### Config

Create an .env file and paste your apikey and your database url like so:

TWELVE_SECRET=xyz
DATABASE_URL=mysql://root:root@localhost/twelve

### Local Installation

Install Rust and Cargo toolchain

https://doc.rust-lang.org/stable/cargo/getting-started/installation.html

Install the sqlx-cli tool
```
cargo install sqlx-cli
```

Create the database and run migrations
```
sqlx database create
sqlx migrate run
```

Install the program
```
cargo install --path /cli
cargo install --path /api
```

### Command help

Usage: twelve --symbol <SYMBOL> --start <START> --end <END> --interval <INTERVAL>

Running command example
```
twelve-cli -s=BTC/USD --start=2023-05-09 --end='2023-05-10 23:59:59' -i=5min
```

### API deployment example

https://gist.github.com/belst/ff36c5f3883f7bf9b06c379d0a7bed9e

### Local Docker Setup

Build
```
docker build -t twelve .
```

Run
```
docker run -it --rm --name twelve-rs twelve
```