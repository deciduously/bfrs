# BFRS
[![Build Status](https://travis-ci.org/deciduously/bfrs.svg?branch=master)](https://travis-ci.org/deciduously/bfrs)

Quick 'n dirty [BrainFuck](https://en.wikipedia.org/wiki/Brainfuck) interpereter in Rust.
## Dependencies
Rust
## Usage
Use 'cargo build' to generate the executable `bfrsc`, which accepts a Brainfuck program as its first argument.  Use for example `cargo run resource/sierpinksi.b` to do it all in one fell swoop.  Use `cargo test` to run the tests.

On my machine (AMD A10-5750M) it runs `benchmark.b` in 13.21 seconds and [`mandlebrot.b`](https://github.com/kostya/benchmarks/blob/master/brainfuck2/mandel.b) in 221.16 seconds.
## Acknowledgements
This [crazy page](http://www.hevanet.com/cristofd/brainfuck/) from this [crazy guy](http://www.hevanet.com/cristofd/brainfuck/daniel.png).  Thanks, Cristof.  He wrote the sierpinksi one and some of the other tests I've been using.

Benchmark and Mandlebrot programs yanked from [kostya](https://github.com/kostya/benchmarks).