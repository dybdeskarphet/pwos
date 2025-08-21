# 🩹 pwos 

![Rust](https://img.shields.io/badge/rust-nightly-blue?logo=rust) ![GitHub License](https://img.shields.io/github/license/dybdeskarphet/pwos) ![x86_64](https://img.shields.io/badge/platform-x86__64-lightgrey) ![Build Test](https://img.shields.io/github/actions/workflow/status/dybdeskarphet/pwos/rust.yml
)

p(atch)w(ork)os is a minimal toy operating system written in Rust. Built for fun, learning, and breaking stuff on purpose.

## Features

* Boots on x86\_64 (bare metal or QEMU)
* Written in pure Rust (`no_std`)
* Just enough to say "it works"

## How does it look?

<img height="300" src="./assets/2025-08-21.png"/>

As you can see, not much is happening.

## Dependencies

- [`bootimage=0.10.3`](https://crates.io/crates/bootimage) in the `$PATH`

## Usage


- **Build with:** `cargo build`
- **Run with:** `cargo run`
