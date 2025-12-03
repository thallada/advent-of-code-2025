# Advent of Code 2025

Rusty and over-engineered edition.

## Running

By request of AoC creator, I haven't included the input files (e.g. src/input/day01.txt). Log into the Advent of Code site and save the inputs there to the src/input/ folder.

Then to run: `cargo run`.

To run in super-fast prod mode: `cargo run --release`.

To run with debug logs enabled: `RUST_LOG=debug cargo run`.

To run the tests against included test input files: `RUST_LOG=debug cargo test -- --no-capture`.

## Benchmarks

Because this is over-engineered, I've included benchmarks for each day's solution. Because, why not?

To run benchmarks: `cargo bench`.

### Results

These were all run on my personal machine, an AMD Ryzen 9 3900X 12-Core Processor with 32 GB RAM, on Linux (WSL).

Timings are given as: [lower-bound **best-estimate** upper-bound]

| Day | Part 1 | Part 2 |
|-----|--------|--------|
| 01  | [101.34 µs **101.95 µs** 102.61 µs] | [105.90 µs **106.40 µs** 106.95 µs] |
