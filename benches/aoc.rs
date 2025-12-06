use criterion::{Criterion, criterion_group, criterion_main};
use pprof::criterion::{Output, PProfProfiler};

const PPROF_SAMPLING_FREQ_HZ: i32 = 997;

macro_rules! bench_days {
    ($($day_num:literal => $day_mod:ident),* $(,)?) => {
        $(
            mod $day_mod {
                use super::*;
                use aoc::$day_mod;

                pub fn part1(c: &mut Criterion) {
                    c.bench_function(concat!(stringify!($day_mod), " part1"), |b| {
                        b.iter(|| $day_mod::part1($day_mod::INPUT))
                    });
                }

                pub fn part2(c: &mut Criterion) {
                    c.bench_function(concat!(stringify!($day_mod), " part2"), |b| {
                        b.iter(|| $day_mod::part2($day_mod::INPUT))
                    });
                }
            }
        )*

        criterion_group! {
            name = benches;
            config = Criterion::default().with_profiler(PProfProfiler::new(PPROF_SAMPLING_FREQ_HZ, Output::Flamegraph(None)));
            targets = $($day_mod::part1, $day_mod::part2),*
        }
        criterion_main!(benches);
    };
}

aoc::all_days!(bench_days);
