use aoc::day01;
use aoc::day02;
use criterion::{Criterion, criterion_group, criterion_main};

fn day01_benchmark(c: &mut Criterion) {
    c.bench_function("day01 part1", |b| b.iter(|| day01::part1(day01::INPUT)));
    c.bench_function("day01 part2", |b| b.iter(|| day01::part2(day01::INPUT)));
}

fn day02_benchmark(c: &mut Criterion) {
    c.bench_function("day02 part1", |b| b.iter(|| day02::part1(day02::INPUT)));
    c.bench_function("day02 part2", |b| b.iter(|| day02::part2(day02::INPUT)));
}

criterion_group!(benches, day01_benchmark, day02_benchmark);
criterion_main!(benches);
