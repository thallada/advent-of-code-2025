use std::str::FromStr;

use color_eyre::{
    Result,
    eyre::{Error, eyre},
};
use tracing::{debug, info, instrument};

pub const INPUT: &str = include_str!("input/input.txt");

const LOCK_SIZE: i32 = 100;
const LOCK_STARTING_POSITION: i32 = 50;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(eyre!("Invalid direction")),
        }
    }
}

#[instrument(skip(input))]
pub fn part1(input: &str) -> Result<i32> {
    let mut dial = LOCK_STARTING_POSITION;
    let mut visited_zero_count = 0;
    for line in input.trim().split('\n') {
        let dir: &Direction = &line[0..1].parse()?;
        let clicks: i32 = line[1..].parse()?;
        match dir {
            Direction::Left => {
                debug!("Turn left {} clicks", clicks);
                dial = (dial - clicks).rem_euclid(LOCK_SIZE);
                debug!("Dial is now {}", dial);
            }
            Direction::Right => {
                debug!("Turn right {} clicks", clicks);
                dial = (dial + clicks).rem_euclid(LOCK_SIZE);
                debug!("Dial is now {}", dial);
            }
        }
        if dial == 0 {
            visited_zero_count += 1;
            debug!("Visited zero, count is now {}", visited_zero_count);
        }
    }
    Ok(visited_zero_count)
}

#[instrument(skip(input))]
pub fn part2(input: &str) -> Result<i32> {
    let mut dial = LOCK_STARTING_POSITION;
    let mut visited_zero_count = 0;
    for line in input.trim().split('\n') {
        let dir: &Direction = &line[0..1].parse()?;
        let clicks: i32 = line[1..].parse()?;
        match dir {
            Direction::Left => {
                debug!("Turn left {} clicks", clicks);
                let end = dial - clicks;
                let passing_zero_count =
                    (dial - 1).div_euclid(LOCK_SIZE) - (end - 1).div_euclid(LOCK_SIZE);
                if passing_zero_count > 0 {
                    visited_zero_count += passing_zero_count;
                    debug!(
                        "Passed zero {} times, count is now {}",
                        passing_zero_count, visited_zero_count
                    );
                }
                dial = end.rem_euclid(LOCK_SIZE);
                debug!("Dial is now {}", dial);
            }
            Direction::Right => {
                debug!("Turn right {} clicks", clicks);
                let end = dial + clicks;
                let passing_zero_count = end.div_euclid(LOCK_SIZE);
                if passing_zero_count > 0 {
                    visited_zero_count += passing_zero_count;
                    debug!(
                        "Passed zero {} times, count is now {}",
                        passing_zero_count, visited_zero_count
                    );
                }
                dial = end.rem_euclid(LOCK_SIZE);
                debug!("Dial is now {}", dial);
            }
        }
    }
    Ok(visited_zero_count)
}

pub fn solve() -> Result<()> {
    info!("Day 1");
    {
        let _span = tracing::info_span!("day01").entered();
        let p1 = part1(INPUT)?;
        info!("Part 1: {}", p1);
        let p2 = part2(INPUT)?;
        info!("Part 2: {}", p2);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_parse_direction() {
        let result = "L".parse::<Direction>();
        assert!(matches!(result.unwrap(), Direction::Left));
    }

    #[test]
    fn test_invalid_direction() {
        let result = "U".parse::<Direction>();
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid direction"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 6);
    }
}
