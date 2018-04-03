# BFRS
[![Build Status](https://travis-ci.org/deciduously/bfrs.svg?branch=master)](https://travis-ci.org/deciduously/bfrs)

Quick 'n dirty [BrainFuck](https://en.wikipedia.org/wiki/Brainfuck) interpereter in Rust.
## Dependencies
Rust 1.25.0 or higher (I jumped on those nested import groups).  Or download the 64-bit Linux binary from [releases](https://github.com/deciduously/bfrs/releases).
## Usage
`bfrs` accepts a brainfuck file as its first argument, and optionally a second argument of either `-d` or `--debug` to dump machine state after each instruction, which is obnoxious. For example: `bfrs resource/benchmark.b -d`.
## Development
Use [cargo](https://doc.rust-lang.org/stable/cargo) run, test, install, --release, etc.
## Performance
Middling.

On my machine (AMD A10-5750M) it runs `benchmark.b` in 7.02 seconds and [`mandel.b`](https://github.com/kostya/benchmarks/blob/master/brainfuck2/mandel.b) in 102.20 seconds.

First stab at it was even worse, 13.21 and 221.16 respectively, so, you know, gainz.
## Acknowledgements
This [crazy page](http://www.hevanet.com/cristofd/brainfuck/) from this [crazy guy](http://www.hevanet.com/cristofd/brainfuck/daniel.png).  Thanks, Cristof.  He wrote the sierpinksi one and some of the other tests I've been using.

Benchmark and Mandlebrot programs yanked from [kostya](https://github.com/kostya/benchmarks).
