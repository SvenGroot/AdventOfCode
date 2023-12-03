// https://adventofcode.com/2023/day/3

use std::collections::{HashMap, HashSet};

use aoc::{
    grid::{GridBuilder, Point},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Sum all numbers that are adjacent to a symbol.
fn part1(input: AocInput) -> usize {
    let engine = GridBuilder::from_input(input).build();
    let mut sum = 0;
    for (row, values) in engine.rows().enumerate() {
        let mut current_number: usize = 0;
        let mut has_symbol = false;
        for (col, value) in values.iter().enumerate() {
            if value.is_ascii_digit() {
                current_number *= 10;
                current_number += (value - b'0') as usize;
                has_symbol = has_symbol
                    || engine
                        .all_neighbors((row, col).into())
                        .any(|pos| !engine[pos].is_ascii_digit() && engine[pos] != b'.');
            } else {
                if current_number != 0 && has_symbol {
                    println!("{current_number}");
                    sum += current_number;
                }

                current_number = 0;
                has_symbol = false;
            }
        }

        if current_number != 0 && has_symbol {
            println!("{current_number}");
            sum += current_number;
        }
    }

    sum
}

// Find all * symbols adjacent to exactly two numbers, multiply those two numbers together, sum
// them.
fn part2(input: AocInput) -> usize {
    let engine = GridBuilder::from_input(input).build();
    let mut current_symbols = HashSet::new();
    let mut gear_symbols: HashMap<Point, Vec<usize>> = HashMap::new();
    for (row, values) in engine.rows().enumerate() {
        let mut current_number: usize = 0;
        current_symbols.clear();
        for (col, value) in values.iter().enumerate() {
            if value.is_ascii_digit() {
                current_number *= 10;
                current_number += (value - b'0') as usize;
                for nb in engine.all_neighbors((row, col).into()) {
                    if engine[nb] == b'*' {
                        current_symbols.insert(nb);
                    }
                }
            } else {
                for symbol in &current_symbols {
                    gear_symbols
                        .entry(*symbol)
                        .and_modify(|v| v.push(current_number))
                        .or_insert_with(|| vec![current_number]);
                }

                current_number = 0;
                current_symbols.clear();
            }
        }

        for symbol in &current_symbols {
            gear_symbols
                .entry(*symbol)
                .and_modify(|v| v.push(current_number))
                .or_insert_with(|| vec![current_number]);
        }
    }

    gear_symbols
        .iter()
        .filter(|(_, numbers)| (numbers.len() == 2))
        .map(|(_, numbers)| numbers[0] * numbers[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(4361, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(467835, part2(AocInput::from_sample()));
    }
}
