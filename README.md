# Advent of Code 2025

Rusty and over-engineered edition.

## Running

By request of AoC creator, I haven't included the input files (e.g. src/input/day01.txt). Log into the Advent of Code site and save the inputs there to the src/input/ folder.

To run all days: `cargo run`.

To run a specific day and/or part: `cargo run -- --day 1 --part 1`.

To run in super-fast prod mode: `cargo run --release`.

To run with debug logs enabled: `RUST_LOG=debug cargo run`.

To run all the tests against included test input files: `RUST_LOG=debug cargo test -- --no-capture`.

To run the tests for a specific day and/or part: `RUST_LOG=debug cargo test day01::test::test_part1 -- --no-capture`.

## Benchmarks

Because this is over-engineered, I've included benchmarks for each day's solution. Because, why not?

To run benchmarks: `cargo bench`. Or a specific day and/or part: `cargo bench -- "day02 part1"`.

### Results

These were all run on my personal machine, an AMD Ryzen 9 3900X 12-Core Processor with 32 GB RAM, on Linux (WSL), with nightly rust.

Timings are given as: [lower-bound **best-estimate** upper-bound]

| Day | Part 1 | Part 2 |
|-----|--------|--------|
| 01  | [79.998 µs **80.349 µs** 80.721 µs] | [76.289 µs **76.616 µs** 76.950 µs] |
| 02  | [2.0386 ms **2.0483 ms** 2.0584 ms] | [2.0823 ms **2.0918 ms** 2.1015 ms] |
| 03  | [45.711 µs **45.937 µs** 46.177 µs] | [267.18 µs **267.95 µs** 268.75 µs] |
| 04  | [143.40 µs **144.00 µs** 144.73 µs] | [1.6165 ms **1.6258 ms** 1.6355 ms] |
| 05  | [187.25 µs **188.93 µs** 190.74 µs] | [63.809 µs **64.204 µs** 64.606 µs] |
| 06  | [128.44 µs **129.44 µs** 130.52 µs] | [165.05 µs **165.70 µs** 166.36 µs] |
| 07  | [83.803 µs **84.601 µs** 85.435 µs] | [81.456 µs **82.360 µs** 83.386 µs] |

## Profiling

To aid in increasing performance, the `pprof` crate can be used to generate flamegraphs off of the benchmarks.

To run profiling across all benchmarks: `cargo bench --bench aoc -- --profile-time 10`.

To run profile the benchmark for a specific day and/or part: `cargo bench --bench aoc -- --profile-time 30 "day01 part1"`.

The flamegraphs will be generated in `target/criterion/<benchmark_name>/profile/flamegraph.svg`.
