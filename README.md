# webhook

A simple webhook server. Nothing else.

## Features

- Execute command and scripts via HTTP requests
- Basic and Bearer authentication
- n-to-n relations between authentication tokens and hooks
- Hot reloading of hook and auth configuration 

## Example Configuration

*TODO™*

## Setup

### Pre-compiled binaries

You can download the latest pre-compiled release binaries from the [releases page](https://github.com/zekroTJA/webhook/releases).

### From source

Make sure to have the latest version of the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

Then, you can install the binary directly via Cargo.

```
$ cargo install --git https://github.com/zekroTJA/webhook
```

Alternatively, you can also manually clone the repository and build it with the following command.

```
$ cargo build --release --locked
```

The release binary can be found in the `target/release` directory.

## systemd

You can use the provided [service configuration](systemd/webhook.service) to set up webhook as systemd service unit.

## Docker

You can also host the service using the provided [Docker image](https://github.com/zekroTJA/webhook/pkgs/container/webhook).