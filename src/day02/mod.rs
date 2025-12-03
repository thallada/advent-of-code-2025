use std::{fmt::Display, str::FromStr};

use color_eyre::{
    Result,
    eyre::{Error, OptionExt, eyre},
};
use itertools::Itertools;
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

fn is_valid_product_id(id: i64) -> Result<bool> {
    let digits: Vec<u32> = id
        .to_string()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_eyre("Invalid product id: contains a non-decimal digit")
        })
        .collect::<Result<Vec<u32>>>()?;
    if digits.len() % 2 != 0 {
        return Ok(true);
    }
    Ok(digits[..digits.len() / 2] != digits[digits.len() / 2..])
}

fn is_valid_product_id2(id: i64) -> Result<bool> {
    let digits: Vec<u32> = id
        .to_string()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_eyre("Invalid product id: contains a non-decimal digit")
        })
        .collect::<Result<Vec<u32>>>()?;
    let mut chunk_size = digits.len() / 2;
    loop {
        if chunk_size == 0 {
            break;
        }
        if digits
            .chunks(chunk_size)
            .tuple_windows()
            .all(|(a, b)| a == b)
        {
            return Ok(false);
        }
        chunk_size -= 1;
    }
    Ok(true)
}

impl ProductRange {
    fn invalid_ids(self) -> Result<Vec<i64>> {
        let mut invalid_ids = vec![];

        for id in self {
            if !is_valid_product_id(id)? {
                invalid_ids.push(id);
            }
        }

        debug!("Invalid IDs: {:?}", &invalid_ids);
        Ok(invalid_ids)
    }

    fn invalid_ids2(self) -> Result<Vec<i64>> {
        let mut invalid_ids = vec![];

        for id in self {
            if !is_valid_product_id2(id)? {
                invalid_ids.push(id);
            }
        }

        debug!("Invalid IDs: {:?}", &invalid_ids);
        Ok(invalid_ids)
    }
}

#[instrument(skip(input))]
pub fn part1(input: &str) -> Result<i64> {
    let mut total_invalid = 0;
    for range in input.trim().split(',') {
        let _span = debug_span!("range", range = %range).entered();
        let range: ProductRange = range.parse()?;
        total_invalid += range.invalid_ids()?.iter().sum::<i64>();
    }
    Ok(total_invalid)
}

#[instrument(skip(input))]
pub fn part2(input: &str) -> Result<i64> {
    let mut total_invalid = 0;
    for range in input.trim().split(',') {
        let _span = debug_span!("range", range = %range).entered();
        let range: ProductRange = range.parse()?;
        total_invalid += range.invalid_ids2()?.iter().sum::<i64>();
    }
    Ok(total_invalid)
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
