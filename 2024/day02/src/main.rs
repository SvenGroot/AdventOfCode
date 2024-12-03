// https://adventofcode.com/2024/day/2

use std::{cmp::Ordering, str::FromStr};

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// How many reports are safe (increasing or decreasing with a difference no greater than 3).
fn part1(input: AocInput) -> usize {
    input
        .filter(|line| {
            let report: Report = line.parse().unwrap();
            report.is_safe()
        })
        .count()
}

// Same, but with the "dampener" which allows removing any one level from an unsafe report.
fn part2(input: AocInput) -> usize {
    input
        .filter(|line| {
            let report: Report = line.parse().unwrap();
            report.is_safe_with_dampener()
        })
        .count()
}

#[derive(Clone)]
struct Report {
    levels: Vec<isize>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut order = Order::Unknown;
        for window in self.levels.windows(2) {
            let first = window[0];
            let second = window[1];

            if !Self::is_pair_safe(&mut order, first, second) {
                return false;
            }
        }

        true
    }

    fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        // Try to remove each level and see if it becomes safe.
        // This is just brute force, and could easily be done without clone or more clever, but it's
        // not needed.
        for i in 0..self.levels.len() {
            let mut clone = self.clone();
            clone.levels.remove(i);
            if clone.is_safe() {
                return true;
            }
        }

        false
    }

    fn is_pair_safe(order: &mut Order, first: isize, second: isize) -> bool {
        match *order {
            Order::Unknown => match first.cmp(&second) {
                Ordering::Less => *order = Order::Increasing,
                Ordering::Greater => *order = Order::Decreasing,
                Ordering::Equal => return false,
            },
            Order::Increasing if first >= second => return false,
            Order::Decreasing if first <= second => return false,
            _ => {}
        }

        (second - first).abs() <= 3
    }
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report {
            levels: s
                .split_ascii_whitespace()
                .map(|level| level.parse::<isize>().unwrap())
                .collect(),
        })
    }
}

#[derive(PartialEq, Eq)]
enum Order {
    Unknown,
    Increasing,
    Decreasing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(AocInput::from_sample()));
    }
}
