// https://adventofcode.com/2024/day/2

use aoc::{input::AocInput, iterator::IteratorExt};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    input
        .filter(|line| {
            let mut order = Order::Unknown;
            for window in line
                .split_ascii_whitespace()
                .map(|level| level.parse::<isize>().unwrap())
                .into_vec()
                .windows(2)
            {
                let first = window[0];
                let second = window[1];
                use std::cmp::Ordering;

                match order {
                    Order::Unknown => match first.cmp(&second) {
                        Ordering::Less => order = Order::Increasing,
                        Ordering::Greater => order = Order::Decreasing,
                        Ordering::Equal => return false,
                    },
                    Order::Increasing if first >= second => return false,
                    Order::Decreasing if first <= second => return false,
                    _ => {}
                }

                if (second - first).abs() > 3 {
                    return false;
                }
            }

            true
        })
        .count()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
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
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
