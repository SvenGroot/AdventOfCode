// https://adventofcode.com/2023/day/12

use std::str::FromStr;

use aoc::input::AocInput;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Count the possible valid arrangements of damaged and operational springs.
fn part1(input: AocInput) -> usize {
    input
        .parsed::<SpringRow>()
        .map(SpringRow::count_arrangements)
        .sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct SpringRow {
    springs: Vec<SpringState>,
    groups: Vec<usize>,
}

impl SpringRow {
    fn count_arrangements(mut self) -> usize {
        let unknown_indices = self
            .springs
            .iter()
            .enumerate()
            .filter_map(|(index, spring)| (*spring == SpringState::Unknown).then_some(index))
            .collect_vec();

        let max = 1usize << unknown_indices.len();
        let mut arrangments = 0;
        for mut value in 0..max {
            for index in &unknown_indices {
                self.springs[*index] = if value & 1 == 1 {
                    SpringState::Damaged
                } else {
                    SpringState::Operational
                };

                value >>= 1;
            }

            if self.is_valid() {
                arrangments += 1;
            }
        }

        arrangments
    }

    fn is_valid(&self) -> bool {
        // This could easily be done without allocating.
        let actual = self
            .springs
            .iter()
            .group_by(|spring| **spring)
            .into_iter()
            .filter(|(spring, _)| *spring == SpringState::Damaged)
            .map(|(_, group)| group.count())
            .collect_vec();

        actual == self.groups
    }
}

impl FromStr for SpringRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s.split_once(' ').unwrap();
        let springs = springs
            .bytes()
            .map(|b| match b {
                b'.' => SpringState::Operational,
                b'#' => SpringState::Damaged,
                b'?' => SpringState::Unknown,
                _ => unreachable!(),
            })
            .collect();

        let groups = groups
            .split(',')
            .map(|group| group.parse().unwrap())
            .collect();

        Ok(Self { springs, groups })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
