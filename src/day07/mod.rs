use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use color_eyre::{Result, eyre::eyre};
use tracing::{debug, instrument};

pub const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Source,
    Splitter,
    Beam(usize),
    Empty,
}

impl FromStr for Cell {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Cell::Source),
            "^" => Ok(Cell::Splitter),
            "|" => Ok(Cell::Beam(1)),
            "." => Ok(Cell::Empty),
            _ => Err(eyre!("Invalid cell character: {}", s)),
        }
    }
}

impl Cell {
    fn from_byte(b: u8) -> Result<Self, color_eyre::Report> {
        match b {
            b'S' => Ok(Cell::Source),
            b'^' => Ok(Cell::Splitter),
            b'|' => Ok(Cell::Beam(1)),
            b'.' => Ok(Cell::Empty),
            _ => Err(eyre!("Invalid cell byte: {}", b)),
        }
    }
}

struct Grid<const R: usize, const C: usize> {
    cells: [[Cell; C]; R],
    splits: usize,
}

impl<const R: usize, const C: usize> FromStr for Grid<R, C> {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = [[Cell::Empty; C]; R];

        for (row, line) in s.lines().enumerate() {
            for (col, byte) in line.bytes().enumerate() {
                cells[row][col] = Cell::from_byte(byte)?;
            }
        }

        Ok(Grid { cells, splits: 0 })
    }
}

impl<const R: usize, const C: usize> Display for Grid<R, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..R {
            for col in 0..C {
                let symbol = match self.cells[row][col] {
                    Cell::Source => 'S',
                    Cell::Splitter => '^',
                    Cell::Beam(t) => (b'0' + t as u8) as char,
                    Cell::Empty => '.',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const R: usize, const C: usize> Grid<R, C> {
    fn emit_beam(&mut self) {
        for row in 1..R {
            for col in 0..C {
                if matches!(self.cells[row][col], Cell::Beam(_)) {
                    // already filled by a splitter to the left
                    continue;
                }
                if matches!(self.cells[row - 1][col], Cell::Source | Cell::Beam(_)) {
                    let timelines = match self.cells[row - 1][col] {
                        Cell::Source => 1,
                        Cell::Beam(t) => t,
                        _ => unreachable!(),
                    };
                    if self.cells[row][col] == Cell::Splitter {
                        // assumption: splitter always has empty cells on left and right
                        let left = self.cells[row][col - 1];
                        let left_timelines = match self.cells[row - 1][col - 1] {
                            Cell::Source => 1,
                            Cell::Beam(t) => t,
                            _ => 0,
                        };
                        if let Cell::Beam(left_t) = left {
                            self.cells[row][col - 1] = Cell::Beam(timelines + left_t);
                        } else {
                            self.cells[row][col - 1] = Cell::Beam(timelines + left_timelines);
                        }
                        let right = self.cells[row][col + 1];
                        let right_timelines = match self.cells[row - 1][col + 1] {
                            Cell::Source => 1,
                            Cell::Beam(t) => t,
                            _ => 0,
                        };
                        if let Cell::Beam(right_t) = right {
                            self.cells[row][col + 1] = Cell::Beam(timelines + right_t);
                        } else {
                            self.cells[row][col + 1] = Cell::Beam(timelines + right_timelines);
                        }
                        self.splits += 1;
                    } else {
                        self.cells[row][col] = Cell::Beam(timelines);
                    }
                }
            }
            debug!("After row {}:\n{}", row, self);
        }
    }
}

fn solve_part1<const R: usize, const C: usize>(input: &str) -> Result<usize> {
    let mut grid = Grid::<R, C>::from_str(input)?;
    grid.emit_beam();
    Ok(grid.splits)
}

#[instrument(skip(input))]
pub fn part1(input: &str) -> Result<usize> {
    solve_part1::<142, 141>(input)
}

fn solve_part2<const R: usize, const C: usize>(input: &str) -> Result<usize> {
    let mut grid = Grid::<R, C>::from_str(input)?;
    grid.emit_beam();
    Ok(grid.cells[R - 1]
        .into_iter()
        .map(|c| {
            if let Cell::Beam(timelines) = c {
                timelines
            } else {
                0
            }
        })
        .sum::<usize>())
}

#[instrument(skip(input))]
pub fn part2(input: &str) -> Result<usize> {
    solve_part2::<142, 141>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1::<16, 15>(TEST_INPUT1).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2::<16, 15>(TEST_INPUT1).unwrap(), 40);
    }
}
