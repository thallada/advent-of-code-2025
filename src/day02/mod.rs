use std::{fmt::Display, str::FromStr};

use color_eyre::{
    Result,
    eyre::{Error, eyre},
};
use rayon::prelude::*;
use tracing::{debug, debug_span, instrument};

pub const INPUT: &str = include_str!("input/input.txt");

#[derive(Clone, Debug)]
struct ProductRange(std::ops::RangeInclusive<i64>);

impl FromStr for ProductRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts
            .next()
            .ok_or(eyre!("Invalid product range: no start"))?
            .parse::<i64>()?;
        let end = parts
            .next()
            .ok_or(eyre!("Invalid product range: no end"))?
            .parse::<i64>()?;
        if parts.next().is_some() {
            return Err(eyre!("Invalid product range: too many parts"));
        }
        Ok(ProductRange(start..=end))
    }
}

impl Display for ProductRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0.start(), self.0.end())
    }
}

impl Iterator for ProductRange {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ProductRange {
    fn invalid_ids(self) -> Result<Vec<i64>> {
        let start = *self.0.start();
        let end = *self.0.end();

        let mut invalid_ids = Vec::new();

        // Determine digit ranges we need to check
        let start_digits = if start == 0 { 1 } else { start.ilog10() + 1 };
        let end_digits = if end == 0 { 1 } else { end.ilog10() + 1 };

        for num_digits in start_digits..=end_digits {
            // Skip odd digit counts - they're all valid
            if num_digits % 2 != 0 {
                continue;
            }

            let half_digits = num_digits / 2;
            let half_min = 10_i64.pow(half_digits - 1);
            let half_max = 10_i64.pow(half_digits) - 1;
            let multiplier = 10_i64.pow(half_digits) + 1; // Pre-calculate: half * multiplier = AABB pattern

            // Generate all patterns where first half == second half
            for half in half_min..=half_max {
                let id = half * multiplier;
                if id >= start && id <= end {
                    invalid_ids.push(id);
                }
            }
        }

        debug!("Invalid IDs: {:?}", &invalid_ids);
        Ok(invalid_ids)
    }

    fn invalid_ids2(self) -> Result<Vec<i64>> {
        let start = *self.0.start();
        let end = *self.0.end();

        let mut invalid_ids = std::collections::HashSet::new();

        // Determine digit ranges we need to check
        let start_digits = if start == 0 { 1 } else { start.ilog10() + 1 };
        let end_digits = if end == 0 { 1 } else { end.ilog10() + 1 };

        for num_digits in start_digits..=end_digits {
            // Try all possible chunk sizes that divide evenly
            for chunk_size in 1..=num_digits / 2 {
                if num_digits % chunk_size != 0 {
                    continue;
                }

                let num_chunks = num_digits / chunk_size;
                if num_chunks < 2 {
                    continue;
                }

                // Generate all possible chunk patterns
                let chunk_min = 10_i64.pow(chunk_size - 1);
                let chunk_max = 10_i64.pow(chunk_size) - 1;
                let chunk_power = 10_i64.pow(chunk_size);

                // Calculate multiplier for repeating pattern
                // For ABCABC: chunk * (10^6 + 10^3 + 1) = chunk * 1001001
                let mut multiplier = 0_i64;
                for i in 0..num_chunks {
                    multiplier += chunk_power.pow(i);
                }

                for chunk in chunk_min..=chunk_max {
                    let id = chunk * multiplier;

                    if id >= start && id <= end {
                        invalid_ids.insert(id);
                    }
                }
            }
        }

        let invalid_ids: Vec<i64> = invalid_ids.into_iter().collect();

        debug!("Invalid IDs: {:?}", &invalid_ids);
        Ok(invalid_ids)
    }
}

#[instrument(skip(input))]
pub fn part1(input: &str) -> Result<i64> {
    input
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|range| {
            let _span = debug_span!("range", range = %range).entered();
            let range: ProductRange = range.parse()?;
            Ok(range.invalid_ids()?.iter().sum::<i64>())
        })
        .sum()
}

#[instrument(skip(input))]
pub fn part2(input: &str) -> Result<i64> {
    input
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|range| {
            let _span = debug_span!("range", range = %range).entered();
            let range: ProductRange = range.parse()?;
            Ok(range.invalid_ids2()?.iter().sum::<i64>())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 4174379265);
    }
}
