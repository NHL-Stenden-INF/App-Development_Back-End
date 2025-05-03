# AppDevelopment - Backend

This is the backend for the project App Development. It's written entirely in Rust

[Frontend repo](https://github.com/NHL-Stenden-INF/App-Development_Front-End)

## Installation

Copy the SQLite file `app-dev.db.example` to `app-dev.db`
```bash
cp app-dev.db.example app-dev.db
```
Copy the `.env.example` file to `.env`
```bash
cp .env.example .env
```
Start the Docker container, add the `-d` flag if you wish for Docker to be run in the background
```bash
docker-compose up --build
```

In case you wish to run the program without using Docker, you can install the [Rust toolchain here](https://www.rust-lang.org/tools/install), and run it with Cargo. Cargo will automatically install the needed Rust libraries
```bash
cargo run .
```
*Note: if you run with Cargo, you may need to install an SQLite driver for your system* 

The SQLite file *should* already contain the correct tables, but if that's not the case, there's a `template.sql` file that has the entire table structure of the SQLite file, which can be manually ran in case of unforeseen errors 


## Details

The backend is fully written in Rust, using the [Axum](https://crates.io/crates/axum) framework for handling HTTP requests and the REST API. For more details for the other used libraries, please refer to the Cargo.toml file, which contains the list of all used dependencies