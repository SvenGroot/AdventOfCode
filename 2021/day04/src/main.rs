// https://adventofcode.com/2021/day/4

use std::fmt::Display;

use aoc::{grid::Grid, input::AocInput};
use colored::Colorize;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Determine the score of the winning bingo board.
fn part1(input: AocInput) -> usize {
    let input = input.into_vec();
    let mut game: BingoGame = input.split(|line| line.is_empty()).collect();
    game.play()
}

// Determine the score of the board that wins last.
fn part2(input: AocInput) -> usize {
    let input = input.into_vec();
    let mut game: BingoGame = input.split(|line| line.is_empty()).collect();
    game.play_to_lose()
}

#[derive(Debug)]
struct BingoGame {
    numbers: Vec<usize>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn play(&mut self) -> usize {
        for number in &self.numbers {
            for board in &mut self.boards {
                if board.play(*number) {
                    println!("Winning board:");
                    println!("{board}");
                    return board.get_score(*number);
                }
            }
        }

        panic!("Nobody won");
    }

    fn play_to_lose(&mut self) -> usize {
        let mut last_win = None;
        let mut final_number = 0;
        let mut boards_to_remove = Vec::new();
        for number in &self.numbers {
            boards_to_remove.clear();
            for (index, board) in self.boards.iter_mut().enumerate() {
                if board.play(*number) {
                    last_win = Some(board.clone());
                    final_number = *number;
                    boards_to_remove.push(index);
                }
            }

            for index in boards_to_remove.iter().rev() {
                self.boards.remove(*index);
            }
        }

        let last_win = last_win.unwrap();
        println!("Last winning board:");
        println!("{last_win}");
        last_win.get_score(final_number)
    }
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
struct BingoBoard(Grid<BingoTile>);

impl BingoBoard {
    fn play(&mut self, number: usize) -> bool {
        for (_, cell) in self.0.cells_mut() {
            if cell.0 == number {
                cell.1 = true;
            }
        }

        return self.0.rows().any(|mut row| row.all(|cell| cell.1))
            || self.0.cols().any(|mut col| col.all(|cell| cell.1));
    }

    fn get_score(&self, final_number: usize) -> usize {
        let sum: usize = self
            .0
            .cells()
            .filter_map(|(_, cell)| (!cell.1).then_some(cell.0))
            .sum();

        sum * final_number
    }
}

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
        assert_eq!(4512, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1924, part2(AocInput::from_sample()));
    }
}
