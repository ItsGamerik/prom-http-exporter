# prom-http-exporter: http healthchecker for prometheus

## Description

This program is a scrapable target for prometheus to get http health check data for other servers.
Define mutltiple targets using the configuration, supporting http aswell as https.

## Usage

### Building

1. Download and install the [rust toolchain](https://rustup.rs/) for your operating system
2. clone this repository with [git](https://git-scm.com/)
3. navigate to the directory to which you cloned this repository and run `cargo b --release`

### Running

The compiled binary will be located in `./target/release/prom-http-exporter`.
Make sure that a configuration file `config.toml` is in the same directory or supply a path as a parameter to the program.
An example configuration is supplied in this directory.

You should then be able to reach the server at `http://<your-host>:<your-port>`.

### Docker

TODO

## configuration

Under the `[server]` section, host and port refer to the host and port the server will bind to.
Setting `accept_invalid_certs` to `true` will allow the server to make connections to other servers with invalid (or self-signed) tls certificates.

The section `targets` contains a list of targets that the exporter should connect to when conducting a health check.
