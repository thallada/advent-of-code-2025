use std::{fmt::Display, str::FromStr};

use color_eyre::{
    Result,
    eyre::{Error, eyre},
};
use tracing::{debug, instrument};

pub const INPUT: &str = include_str!("input/input.txt");

#[derive(Clone, Debug)]
struct FreshRange(pub std::ops::RangeInclusive<i64>);

impl FromStr for FreshRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts
            .next()
            .ok_or(eyre!("Invalid fresh range: no start"))?
            .parse::<i64>()?;
        let end = parts
            .next()
            .ok_or(eyre!("Invalid fresh range: no end"))?
            .parse::<i64>()?;
        if parts.next().is_some() {
            return Err(eyre!("Invalid fresh range: too many parts"));
        }
        Ok(FreshRange(start..=end))
    }
}

impl Display for FreshRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0.start(), self.0.end())
    }
}

#[instrument(skip(input))]
pub fn part1(input: &str) -> Result<usize> {
    let mut processing_ranges = true;
    let mut fresh_ranges = Vec::new();
    let mut fresh_ingredients = 0;
    for line in input.trim().lines() {
        if line.is_empty() {
            processing_ranges = false;
            continue;
        }
        if processing_ranges {
            let range = line.parse::<FreshRange>()?;
            debug!(range = %range);
            fresh_ranges.push(range);
        } else {
            let ingredient = line.parse::<i64>()?;
            debug!(ingredient);
            for range in &mut fresh_ranges {
                if range.0.contains(&ingredient) {
                    fresh_ingredients += 1;
                    debug!(fresh_ingredients, "fresh!");
                    break;
                }
            }
        }
    }
    Ok(fresh_ingredients)
}

#[instrument(skip(input))]
pub fn part2(input: &str) -> Result<usize> {
    let mut fresh_ranges: Vec<Option<FreshRange>> = Vec::new();
    for line in input.trim().lines() {
        if line.is_empty() {
            break;
        }
        let range = line.parse::<FreshRange>()?;
        let mut overlap_range = range.clone();
        debug!(range = %range);
        for range_slot in fresh_ranges.iter_mut() {
            if let Some(existing_range) = range_slot {
                if overlap_range.0.end() < existing_range.0.start()
                    || overlap_range.0.start() > existing_range.0.end()
                {
                    continue;
                }
                let start = *overlap_range.0.start().min(existing_range.0.start());
                let end = *overlap_range.0.end().max(existing_range.0.end());
                if start <= end {
                    overlap_range = FreshRange(start..=end);
                    debug!(overlap_range = %overlap_range, existing_range = %existing_range, "merging existing range");
                    *range_slot = None; // this existing range is now completely merged with the current range
                }
            }
        }
        fresh_ranges.push(Some(overlap_range));
    }
    Ok(fresh_ranges
        .iter()
        .flatten()
        .map(|r| (r.0.end() - r.0.start() + 1) as usize)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 14);
    }

    #[test]
    fn test_part2_triple_overlap() {
        assert_eq!(part2("3-4\n2-5\n1-6").unwrap(), 6);
    }
}
