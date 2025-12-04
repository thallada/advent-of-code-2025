use color_eyre::Result;
use tracing::{debug, instrument};

pub const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy)]
struct Battery {
    column: usize,
    joltage: u8,
}

impl Default for Battery {
    fn default() -> Self {
        Battery {
            column: 0,
            joltage: 0,
        }
    }
}

fn largest_output_joltage<const N: usize>(input: &str) -> Result<u64> {
    let mut output_joltage: u64 = 0;
    for line in input.trim().split('\n') {
        let mut batteries: [Battery; N] = [Battery::default(); N];
        let line_len = line.len();
        for (column, joltage) in line.bytes().map(|c| c - b'0').enumerate() {
            let min = N.saturating_sub(line_len - column);
            for i in min..N {
                if joltage > batteries[i].joltage {
                    batteries[i].column = column;
                    batteries[i].joltage = joltage;
                    batteries[i + 1..].fill(Battery::default());
                    break;
                }
            }
        }
        let line_joltage = batteries
            .iter()
            .fold(0u64, |acc, &b| acc * 10 + b.joltage as u64);
        debug!(line, line_joltage);
        output_joltage += line_joltage;
    }
    Ok(output_joltage)
}

#[instrument(skip(input))]
pub fn part1(input: &str) -> Result<u64> {
    largest_output_joltage::<2>(input)
}

#[instrument(skip(input))]
pub fn part2(input: &str) -> Result<u64> {
    largest_output_joltage::<12>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 3121910778619);
    }
}
