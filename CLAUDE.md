# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

- Run a specific algorithm: `cargo run --bin quicksort`
- Run tests for a specific algorithm: `cargo test --bin quicksort`
- Run all tests: `cargo test`
- Run a single test by name: `cargo test test_random` (works across all bins)
- Build all: `cargo build`

## Architecture

Educational Rust project (edition 2021). Each algorithm is an independent binary in `src/bin/` with its own `main()` and `#[cfg(test)]` module. Cargo auto-discovers files in `src/bin/` — no `Cargo.toml` changes needed when adding new algorithms.

To add a new algorithm: create `src/bin/<name>.rs` with a `fn main()`, then run `cargo run --bin <name>`.
