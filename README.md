# Brunoise

Brunoise (brun-wah), is a configurable event server, written in rust, scripted in python.

## 💡 The idea

The basic idea is that we run a web server that has a configuration interface.
From this interface we can then setup event handlers.
These handlers all take the same function signature, and are written as python objects.
This allows the speed of rust to handle the entrypoint and multithreading, while executing each "event" in an isolated python environment with its own GIL.

## Usage

Currently this project is alpha, there is no packaging setup yet.
However, it can be ran using plain old `cargo run`.
See the [developing section](#developing) for more info.

## Developing

When developing, please build and run the server with the `cargo run` command.
Since we are running embedded python, we need to make sure to add the `PYO3_CONFIG_FILE` env var first.

```sh
PYO3_CONFIG_FILE=$(pwd)/pyembedded/pyo3-build-config-file.txt cargo run
```

### Testing

Run the Cargo test suite with:

```sh
PYO3_CONFIG_FILE=$(pwd)/pyembedded/pyo3-build-config-file.txt cargo test
```

## Authors

* Patty C (schlerp) - [schlerp@proton.me](mailto:schlerp@proton.me)
