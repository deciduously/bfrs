# BFRS
[![Build Status](https://travis-ci.org/deciduously/bfrs.svg?branch=master)](https://travis-ci.org/deciduously/bfrs)

Quick 'n dirty [BrainFuck](https://en.wikipedia.org/wiki/Brainfuck) interpereter in Rust.
## Dependencies
Rust
## Usage
Use 'cargo build' to generate the executable `bfrsc`, which accepts a Brainfuck program as its first argument.  Use for example `cargo run resource/hello-world.bf` to do it all in one fell swoop.  The `--release` flag for cargo enables optimizations and disables debug symbols.  Use `cargo test` to run the tests.
