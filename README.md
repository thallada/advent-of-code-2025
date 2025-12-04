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

These were all run on my personal machine, an AMD Ryzen 9 3900X 12-Core Processor with 32 GB RAM, on Linux (WSL).

Timings are given as: [lower-bound **best-estimate** upper-bound]

| Day | Part 1 | Part 2 |
|-----|--------|--------|
| 01  | [101.34 µs **101.95 µs** 102.61 µs] | [105.90 µs **106.40 µs** 106.95 µs] |
| 02  | [2.0990 ms **2.1113 ms** 2.1236 ms] | [2.0954 ms **2.1055 ms** 2.1157 ms] |
| 03  | [38.717 µs **39.002 µs** 39.311 µs] | [175.35 µs **176.59 µs** 177.92 µs] |
