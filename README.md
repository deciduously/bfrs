# BFRS
[![Build Status](https://travis-ci.org/deciduously/bfrs.svg?branch=master)](https://travis-ci.org/deciduously/bfrs)

Quick 'n dirty [BrainFuck](https://en.wikipedia.org/wiki/Brainfuck) interpereter in Rust.
## Dependencies
Rust 1.25.0 or higher (I jumped on those nested import groups).  Or download the 64-bit Linux binary from [releases](https://github.com/deciduously/bfrs/releases).
## Usage
`bfrsc` accepts a brainfuck file as its first argument, and optionally a second argument of either `-d` or `--debug` to dump machine state after each instruction, which is obnoxious. For example: `bfrsc resoure/sierpinski.b -d`.
## Development
Use [cargo](https://doc.rust-lang.org/stable/cargo) run, test, install, --release, etc.
## Performance
Dismal.

On my machine (AMD A10-5750M) it runs `benchmark.b` in 13.21 seconds and [`mandlebrot.b`](https://github.com/kostya/benchmarks/blob/master/brainfuck2/mandel.b) in 221.16 seconds.
## Acknowledgements
This [crazy page](http://www.hevanet.com/cristofd/brainfuck/) from this [crazy guy](http://www.hevanet.com/cristofd/brainfuck/daniel.png).  Thanks, Cristof.  He wrote the sierpinksi one and some of the other tests I've been using.

Benchmark and Mandlebrot programs yanked from [kostya](https://github.com/kostya/benchmarks).
