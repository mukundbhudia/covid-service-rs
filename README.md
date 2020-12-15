# covid-service-rs

A conversion of the (now archived) JavaScript https://github.com/mukundbhudia/covid-service project into a Rust project.

The target environment will be a Raspberry Pi 4 Model B running Ubuntu 20.04 (aarch64-unknown-linux-gnu).

## Pre-requisites
### Rust
* [Install Rust here](https://www.rust-lang.org/tools/install).
* Minimum Supported Rust Version (MSRV) is any 2018 edition.

### For Linux
* Make sure essential build tools (including compliers) are installed: `sudo apt install build-essential`.
* OpenSSL will be needed `sudo apt install libssl-dev`.
* pkg-config will be needed `sudo apt install pkg-config`.

### For OSX
* Xcode and Xcode command line tools: `xcode-select --install`.

### All platforms
* mongoDB - see: https://docs.mongodb.com/manual/installation/ for install instructions for your platform.
* The `log4rs.yml` file must be present in the same directory as the binary for logging configuration to be set.

## Development
* Within the repo directory run `cargo r "DB_URI" "DB_NAME"` where `DB_URI` is your mongo instance and `DB_NAME` is your mongo database. 
* For example `cargo r "mongodb://localhost:27017/" "covid19"` for connection to local mongoDb instance to the database: `covid19`.

## Production
* To make a production build, within the repo directory run `cargo b --release`.
* To run the release build, within the project directory run `target/release/covid-service-rs "DB_URI" "DB_NAME"` with the same arguments as in development.

## Testing
* To run **all** your tests, within the repo directory run `cargo t`. This will run all tests in the `/tests` directory.
* To run test a **specific** test such as `/tests/my_test.rs`, within the repo directory run `cargo t --test my_test`.

## Advanced builds

* Run `rustc --print target-list` to see a list of targets you can build for.
* You can run `rustup target add <my-target>` to add the specific target to your rustup toolchain. This means you can now compile and build for that target.
* Then running `cargo b --release --target=<my-target>` would create the build for that environment.

## Resources & Thanks
* To [Johns Hopkins CSSE](https://github.com/CSSEGISandData/COVID-19) for the hard work providing and collating the data.
* To the wonderful Rust community for their [excellent book](https://doc.rust-lang.org/book/).
