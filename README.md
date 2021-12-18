# covid-service-rs

![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/mukundbhudia/covid-service-rs)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mukundbhudia/covid-service-rs/Rust)

Collects, processes and aggregates live COVID-19, and historical time series data from Johns Hopkins University. The aggregated data is then used to populate a mongoDB database which supplies data to https://github.com/mukundbhudia/covid-api and subsequently https://github.com/mukundbhudia/covid-web.

This is a conversion of the (now archived) JavaScript https://github.com/mukundbhudia/covid-service project into a Rust project.

The target environment (for my personal use case) will be a Raspberry Pi 4 Model B running Ubuntu 20.04 (`aarch64-unknown-linux-gnu`). However any environment that supports the Rust toolchain should suffice. The service will run as a cron task at regular intervals for example every 20 minutes.

**Note**: only Linux & OS X environments with `aarch64` & `x86_64` architectures have been tested though.

## Pre-requisites

### Rust

- [Install Rust here](https://www.rust-lang.org/tools/install).
- Minimum Supported Rust Version (MSRV) is any 2018 edition.

### For Linux

- Make sure essential build tools (including compilers) are installed: `sudo apt install build-essential`.
- OpenSSL will be needed `sudo apt install libssl-dev`.
- pkg-config will be needed `sudo apt install pkg-config`.

### For OSX

- Xcode and Xcode command line tools: `xcode-select --install`.

### All platforms

- mongoDB - see: https://docs.mongodb.com/manual/installation/ for install instructions for your platform.

## Development

- Within the repo directory run `cargo r "DB_URI" "DB_NAME"` where `DB_URI` is your mongo instance and `DB_NAME` is your mongo database.
- For example `cargo r "mongodb://localhost:27017/" "covid19"` for connection to local mongoDb instance to the database: `covid19`.

## Production

- To make a production build, within the repo directory run `cargo b --release`.
- To run the release build, within the project directory run `target/release/covid-service-rs "DB_URI" "DB_NAME"` with the same arguments as in development.

## Testing

- To run **all** your tests, within the repo directory run `cargo t`. This will run all tests in the `/tests` directory.
- To run test a **specific** test such as `/tests/my_test.rs`, within the repo directory run `cargo t --test my_test`.
- To run ignored tests, run `cargo t -- --ignored`.

## Installation

- If there is a [release asset](https://github.com/mukundbhudia/covid-service-rs/releases) for your target platform, you can run a command like `wget -q -O tmp.zip https://github.com/mukundbhudia/covid-service-rs/releases/latest/download/covid-service-rs_aarch64-unknown-linux-gnu.zip && unzip -o tmp.zip && rm tmp.zip && strip covid-service-rs && mv covid-service-rs /home/ubuntu/.cargo/bin/covid-service-rs` to have the binary placed in the cargo bin folder.

- For a fresh build and install on your target environment, run `cargo install --path .` within the repo directory. Then `covid-service-rs` will be moved to your cargo bin directory and will be callable throughout your host environment as a standard binary.

- To install as a cron task, an example entry under `crontab -e` to run the service every 20 minutes would be `*/20 * * * *    /home/my-user-name/.cargo/bin/covid-service-rs "mongodb://localhost:27017/" "covid19"`

- Subsequent upgrades would then only require a `git pull` and `cargo install --path .` to be run again to build and move the binary to cargo bin.

## Advanced builds (cross-compilation)

- Run `rustc --print target-list` to see a list of targets you can build for.
- You can run `rustup target add <my-target>` to add the specific target to your rustup toolchain. This means you can now compile and build for that target.
- Running `rustup show` will show toolchains already installed on your host environment.
- Then running `cargo b --release --target=<my-target>` would create the build for that environment.

For more information on cross-compilation see https://rust-lang.github.io/rustup/cross-compilation.html.

## Target schema

The intended mongo database should comprise of two collections:

- `casesByLocation` which holds data on every country and/or state along with the time series data. The stuct for this collection can be found as `CaseByLocation` in `/src/schema.rs`.
- `totals` which holds global data including global time series data and . The stuct for this collection can be found as `GlobalCaseByLocation` in `/src/schema.rs`.

## Resources & Thanks

- To [Johns Hopkins CSSE](https://github.com/CSSEGISandData/COVID-19) for the hard work providing and collating the data.
- To [Our World in Data](https://github.com/owid/covid-19-data/tree/master/public/data) for their hard work in providing rich, clear and accurate data available on COVID-19.
- To the wonderful Rust community for their [excellent book](https://doc.rust-lang.org/book/).
