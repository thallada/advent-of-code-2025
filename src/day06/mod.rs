use std::str::FromStr;

use color_eyre::{
    Result,
    eyre::{Context, Error, OptionExt, eyre},
};
use itertools::Itertools;
use tracing::{debug, instrument};

pub const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(eyre!("invalid operation: {}", s)),
        }
    }
}

impl Operation {
    fn from_byte(byte: u8) -> Result<Self> {
        match byte {
            b'+' => Ok(Operation::Add),
            b'*' => Ok(Operation::Multiply),
            _ => Err(eyre!("invalid operation byte: {}", byte)),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

#[derive(Debug, Clone)]
struct CephalopodProblem {
    columns: Vec<Vec<u8>>,
    operation: Operation,
    width: u8,
}

#[instrument(skip(input))]
pub fn part1(input: &str) -> Result<u64> {
    let mut lines = input.trim().lines();
    let first_numbers: Vec<u64> = lines
        .next()
        .ok_or_eyre("no first line in input")?
        .split_whitespace()
        .map(|s| s.parse::<u64>().context("parsing number"))
        .collect::<Result<Vec<u64>>>()?;
    let mut problems = first_numbers
        .into_iter()
        .map(|n| Problem {
            numbers: vec![n],
            operation: Operation::Add,
        })
        .collect::<Vec<Problem>>();
    for line in lines {
        let first_byte = line.bytes().nth(0).ok_or_eyre("empty line in input")?;
        if matches!(first_byte, b'*' | b'+') {
            for (index, op_result) in line.split_whitespace().map(|s| s.parse()).enumerate() {
                let op = op_result?;
                problems[index].operation = op;
            }
        } else {
            for (index, num_result) in line.split_whitespace().map(|s| s.parse()).enumerate() {
                let num = num_result?;
                problems[index].numbers.push(num);
            }
        }
    }
    Ok(problems
        .into_iter()
        .map(|problem| match problem.operation {
            Operation::Add => {
                let sum = problem.numbers.iter().sum::<u64>();
                debug!("{} = {}", problem.numbers.iter().join(" + "), sum);
                return sum;
            }
            Operation::Multiply => {
                let product = problem.numbers.iter().product::<u64>();
                debug!("{} = {}", problem.numbers.iter().join(" * "), product);
                return product;
            }
        })
        .sum())
}

#[instrument(skip(input))]
pub fn part2(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let last_line = lines.next_back().ok_or_eyre("no last line in input")?;
    let mut problems = Vec::new();
    let mut spaces: u8 = 1;
    for byte in last_line.bytes().rev() {
        if byte.is_ascii_whitespace() {
            spaces += 1;
        } else {
            problems.push(CephalopodProblem {
                columns: (0..spaces).map(|_| Vec::new()).collect(),
                operation: Operation::from_byte(byte)?,
                width: spaces,
            });
            spaces = 0;
        }
    }
    debug!(last_problem = ?problems[0]);
    debug!(second_last_problem = ?problems[1]);
    problems.reverse();
    for line in lines {
        let bytes = line.as_bytes();
        let mut offset = 0;
        for i in 0..problems.len() {
            if offset + (problems[i].width as usize) > bytes.len() {
                return Err(eyre!("line too short for problem columns"));
            }
            for (cell_col, &byte) in bytes[offset..offset + problems[i].width as usize]
                .iter()
                .enumerate()
            {
                if !byte.is_ascii_whitespace() {
                    problems[i].columns[cell_col].push(byte - b'0');
                }
            }
            offset += problems[i].width as usize + 1;
        }
    }
    debug!(first_problem = ?problems[0]);
    debug!(second_problem = ?problems[1]);
    Ok(problems
        .iter()
        .map(|problem| match problem.operation {
            Operation::Add => {
                let sum: usize = problem
                    .columns
                    .iter()
                    .map(|col| {
                        col.iter()
                            .fold(0usize, |acc, &digit| acc * 10 + (digit as usize))
                    })
                    .sum();
                debug!(
                    "{} = {}",
                    problem
                        .columns
                        .iter()
                        .map(|col| {
                            col.iter()
                                .fold(0usize, |acc, &digit| acc * 10 + (digit as usize))
                                .to_string()
                        })
                        .join(" + "),
                    sum
                );
                sum
            }
            Operation::Multiply => {
                let product: usize = problem
                    .columns
                    .iter()
                    .map(|col| {
                        col.iter()
                            .fold(0usize, |acc, &digit| acc * 10 + (digit as usize))
                    })
                    .product();
                debug!(
                    "{} = {}",
                    problem
                        .columns
                        .iter()
                        .map(|col| {
                            col.iter()
                                .fold(0usize, |acc, &digit| acc * 10 + (digit as usize))
                                .to_string()
                        })
                        .join(" * "),
                    product
                );
                product
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 3263827);
    }
}
