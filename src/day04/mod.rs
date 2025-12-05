use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use color_eyre::{Result, eyre::eyre};
use tracing::{debug, instrument};

pub const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Paper,
    AccessiblePaper,
}

impl FromStr for Cell {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Empty),
            "@" => Ok(Cell::Paper),
            "x" => Ok(Cell::AccessiblePaper),
            _ => Err(eyre!("Invalid cell character: {}", s)),
        }
    }
}

impl Cell {
    fn from_byte(b: u8) -> Result<Self, color_eyre::Report> {
        match b {
            b'.' => Ok(Cell::Empty),
            b'@' => Ok(Cell::Paper),
            b'x' => Ok(Cell::AccessiblePaper),
            _ => Err(eyre!("Invalid cell byte: {}", b)),
        }
    }
}

struct Grid<const R: usize, const C: usize> {
    cells: [[Cell; C]; R],
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

        Ok(Grid { cells })
    }
}

impl<const R: usize, const C: usize> Display for Grid<R, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..R {
            for col in 0..C {
                let symbol = match self.cells[row][col] {
                    Cell::Empty => '.',
                    Cell::Paper => '@',
                    Cell::AccessiblePaper => 'x',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const R: usize, const C: usize> Grid<R, C> {
    fn get_cell(&self, row: isize, col: isize) -> Option<Cell> {
        if row >= 0 && col >= 0 && (row as usize) < R && (col as usize) < C {
            Some(self.cells[row as usize][col as usize])
        } else {
            None
        }
    }

    fn count_accessible_papers(&mut self, replace_with: Cell) -> usize {
        let mut count = 0;
        for row in 0..R {
            for col in 0..C {
                if self.cells[row][col] != Cell::Paper {
                    continue;
                }
                let mut adjacent_papers = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let adjacent = self
                            .get_cell(row as isize + dr, col as isize + dc)
                            .unwrap_or(Cell::Empty);
                        if adjacent == Cell::Paper || adjacent == Cell::AccessiblePaper {
                            adjacent_papers += 1;
                        }
                    }
                }
                if adjacent_papers < 4 {
                    self.cells[row][col] = replace_with;
                    count += 1;
                }
            }
        }
        count
    }
}

fn solve_part1<const R: usize, const C: usize>(input: &str) -> Result<usize> {
    let mut grid = Grid::<R, C>::from_str(input)?;
    debug!("Parsed grid:\n{}", grid);
    let count = grid.count_accessible_papers(Cell::AccessiblePaper);
    debug!("Processed grid:\n{}", grid);
    Ok(count)
}

#[instrument(skip(input))]
pub fn part1(input: &str) -> Result<usize> {
    solve_part1::<135, 135>(input)
}

fn solve_part2<const R: usize, const C: usize>(input: &str) -> Result<usize> {
    let mut grid = Grid::<R, C>::from_str(input)?;
    debug!("Parsed grid:\n{}", grid);
    let mut count = 0;
    loop {
        let removed = grid.count_accessible_papers(Cell::Empty);
        if removed == 0 {
            break;
        }
        debug!("Removed {} in grid:\n{}", removed, grid);
        count += removed;
    }
    Ok(count)
}

#[instrument(skip(input))]
pub fn part2(input: &str) -> Result<usize> {
    solve_part2::<135, 135>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1::<10, 10>(TEST_INPUT1).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2::<10, 10>(TEST_INPUT1).unwrap(), 43);
    }
}
