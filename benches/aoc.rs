use criterion::{Criterion, criterion_group, criterion_main};

macro_rules! bench_days {
    ($($day_num:literal => $day_mod:ident),* $(,)?) => {
        $(
            use aoc::$day_mod;

            fn $day_mod(c: &mut Criterion) {
                c.bench_function(concat!(stringify!($day_mod), " part1"), |b| {
                    b.iter(|| $day_mod::part1($day_mod::INPUT))
                });
                c.bench_function(concat!(stringify!($day_mod), " part2"), |b| {
                    b.iter(|| $day_mod::part2($day_mod::INPUT))
                });
            }
        )*

        criterion_group!(benches, $($day_mod),*);
        criterion_main!(benches);
    };
}

aoc::all_days!(bench_days);
