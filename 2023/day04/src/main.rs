// https://adventofcode.com/2023/day/4

use std::str::FromStr;

use aoc::{input::AocInput, iterator::IteratorExt};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Get the total points for all the scratch cards.
fn part1(input: AocInput) -> usize {
    input.parsed::<Card>().map(|c| c.points()).sum()
}

// For all matches on a card, duplicate the following cards. Then count the total number of cards.
fn part2(input: AocInput) -> usize {
    let mut stacks = input
        .parsed::<Card>()
        .map(|card| CardStack { card, count: 1 })
        .into_vec();

    for i in 0..stacks.len() {
        let stack = &stacks[i];
        let matches = stack.card.matches();

        // Each duplicate is processed the same, so that's the number of copies we're generating.
        let count = stack.count;
        for m in 1..=matches {
            stacks[i + m].count += count;
        }
    }

    stacks.iter().map(|stack| stack.count).sum()
}

struct CardStack {
    card: Card,
    count: usize,
}

struct Card {
    numbers: Vec<u8>,
    winning_numbers: Vec<u8>,
}

impl Card {
    fn points(&self) -> usize {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            2usize.pow((matches - 1) as u32)
        }
    }

    fn matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count()
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, lists) = s.split_once(": ").unwrap();
        let (numbers, winning_numbers) = lists.split_once(" | ").unwrap();
        let numbers = numbers
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        let winning_numbers = winning_numbers
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            numbers,
            winning_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2(AocInput::from_sample()));
    }
}
