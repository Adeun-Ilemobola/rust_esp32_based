# Rust ESP32 Based

A clean beginner-friendly Rust starter template for ESP32 projects using `esp-idf-svc`, `esp-idf-hal`, and the ESP-IDF Rust ecosystem.

This template is meant to be simple, readable, and easy to extend. The goal is to provide a good starting point for ESP32 Rust projects without adding too much abstraction too early.

## Purpose

This project is a personal ESP32 Rust template that can be reused as a starting point for future embedded projects.

It currently focuses on:

- A clean `main.rs` entry point
- A simple `setup()` and `update()` structure
- Basic utility helpers in `utils.rs`
- ESP-IDF logging support
- A project layout that can grow over time

## Project Structure

```text
.
├── .cargo/
│   └── config.toml
├── .github/
│   └── workflows/
│       └── rust_ci.yml
├── src/
│   ├── main.rs
│   └── utils.rs
├── build.rs
├── Cargo.toml
├── rust-toolchain.toml
├── sdkconfig.defaults
└── README.md
```

## Main Design

The project is designed to feel similar to an embedded or Arduino-style flow:

```rust
fn main() {
    setup();

    loop {
        update();
    }
}
```

The idea is simple:

- `main()` starts the program
- `setup()` runs once
- `update()` runs repeatedly
- `utils.rs` stores reusable helper functions

## Current Utilities

The template includes basic utility functions such as:

- Mapping a value from one range to another
- Constraining/clamping values
- Basic min/max helper functions

These utilities are intentionally simple so the template stays easy to understand for beginners.

## Requirements

Before using this template, make sure you have the ESP Rust toolchain installed and configured.

You will generally need:

- Rust
- ESP Rust toolchain
- ESP-IDF dependencies
- `espflash` or a similar flashing tool
- A supported ESP32 board

For ESP-IDF Rust setup, follow the official ESP Rust template/setup guide:

https://github.com/esp-rs/esp-idf-template

## Build

To build the project:

```bash
cargo build
```

The first build can take a long time because Cargo needs to download and compile the ESP-IDF Rust dependency tree. Later builds should be much faster because Cargo reuses cached dependencies.

## Flash

Depending on your setup, you can flash with:

```bash
cargo run
```

or with `espflash` directly:

```bash
espflash flash target/<target-name>/debug/<binary-name>
```

The exact command may depend on your board, serial port, and local ESP Rust configuration.

## Notes

This project is intentionally kept small for now. More modules and abstractions will be added gradually as the template becomes more mature.

Planned future additions may include:

- LED helpers
- Button helpers
- Servo helpers
- Buzzer helpers
- Sensor abstractions
- Cleaner app structure
- Optional build/flash scripts
- Better documentation for beginners

## Status

Early starter template. Still experimental and actively being improved.

## License

No license has been selected yet.
