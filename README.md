# covid-service-rs

Aim: to convert the JavaScript https://github.com/mukundbhudia/covid-service project into this Rust project.
The target environment will be a Raspberry Pi 4 Model B running Ubuntu 20.04 (aarch64-unknown-linux-gnu).

## Pre-requisites
### Linux
* Make sure essential build tools (including compliers) are installed: `sudo apt install build-essential`
* OpenSSL will be needed `sudo apt install libssl-dev`
* pkg-config will be needed `sudo apt install pkg-config`

### OSX
* Xcode and Xcode command line tools: `xcode-select --install`

### All platforms
* mongoDB - see: https://docs.mongodb.com/manual/installation/ for install instructions for your platform

## Development

* Within the repo directory run `cargo run`.
* To make a production build, within the repo directory run `cargo build --release`.
* To run the release build, within the project directory run `target/release/covid-service-rs`.

## Tests
* To run **all** your tests, within the repo directory run `cargo test --tests`. This will run all tests in the `/tests` directory.

* To run test a **specific** test such as `/tests/my_test.rs`, within the repo directory run `cargo test --test my_test`.

## Advanced builds

* Run `rustc --print target-list` to see a list of targets you can build for.
* You can run `rustup target add <my-target>` to add the specific target to your rustup toolchain. This means you can now compile and build for that target.
* Then running `cargo b --release --target=<my-target>` would create the build for that environment.

### Resources
* To [Johns Hopkins CSSE](https://github.com/CSSEGISandData/COVID-19) for the hard work providing and collating the data.
