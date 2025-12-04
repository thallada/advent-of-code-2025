// Single source of truth for all implemented days
// Add new days here and they'll automatically be available in both the runner and benchmarks

#[macro_export]
macro_rules! all_days {
    ($macro_name:path) => {
        $macro_name! {
            1 => day01,
            2 => day02,
            3 => day03,
        }
    };
}
