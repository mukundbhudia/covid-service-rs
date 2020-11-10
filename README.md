# covid-service-rs

Aim: to convert the JavaScript https://github.com/mukundbhudia/covid-service project into this Rust project.
The target environment will be a Raspberry Pi 4 Model B running Ubuntu 20.04.

## Pre-requisites
### Linux
* Within Raspberry PI, compilers are needed `sudo apt install build-essential`
* OpenSSL needed `sudo apt install libssl-dev`
* pkg-config needed `sudo apt install pkg-config`

### OSX
* Xcode and Xcode command line tools: `xcode-select --install`

### All platforms
* mongoDB - see: https://docs.mongodb.com/manual/installation/ for install instructions for your platform

## Development

* Within the repo directory run `cargo run`.

* To make a production build, within the repo directory run `cargo build --release`.

## Usage

Once a production release has been built (see Development), within the repo directory run `cargo run` (with arguments just like in the Development section).

### Resources
* To [Johns Hopkins CSSE](https://github.com/CSSEGISandData/COVID-19) for the hard work providing and collating the data.
