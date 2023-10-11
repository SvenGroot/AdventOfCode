// https://adventofcode.com/2021/day/4

use std::fmt::Display;

use aoc::{
    grid::{Grid, Point},
    input::AocInput,
};
use colored::Colorize;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    let input = input.into_vec();
    let mut game: BingoGame = input.split(|line| line.is_empty()).collect();
    game.boards[0].0[Point::new(0, 0)].1 = true;
    println!("{game}");
    0
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[derive(Debug)]
struct BingoGame {
    numbers: Vec<usize>,
    boards: Vec<BingoBoard>,
}

impl<'a> FromIterator<&'a [String]> for BingoGame {
    fn from_iter<T: IntoIterator<Item = &'a [String]>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let numbers = iter.next().unwrap()[0]
            .split(',')
            .map(|item| item.parse().unwrap())
            .collect();

        let boards = iter.map(|slice| slice.iter().collect()).collect();
        BingoGame { numbers, boards }
    }
}

impl Display for BingoGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for board in &self.boards {
            writeln!(f, "{}", board)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct BingoTile(usize, bool);

impl Display for BingoTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 {
            write!(f, "{:2}", self.0.to_string().bold().underline())
        } else {
            write!(f, "{:2}", self.0)
        }
    }
}

#[derive(Debug)]
struct BingoBoard(Grid<BingoTile>);

impl<'a> FromIterator<&'a String> for BingoBoard {
    fn from_iter<T: IntoIterator<Item = &'a String>>(iter: T) -> Self {
        BingoBoard(
            iter.into_iter()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|value| BingoTile(value.parse().unwrap(), false))
                        .collect()
                })
                .collect(),
        )
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_row = 0;
        for (pos, value) in self.0.cells() {
            if pos.row() != last_row {
                writeln!(f)?;
                last_row = pos.row();
            }

            if pos.col() != 0 {
                write!(f, " ")?;
            }

            write!(f, "{}", value)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
