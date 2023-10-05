// https://adventofcode.com/2022/day/25

use core::panic;
use std::{fmt::Display, path::Path};

use aoc::{aoc_input, get_input};
use radix_fmt::radix_5;

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(path));
    // This puzzle had no part 2.
}

fn part1(path: impl AsRef<Path>) -> SnafuNumber {
    get_input(path)
        .map(|line| usize::from(SnafuNumber(line)))
        .sum::<usize>()
        .into()
}

#[derive(Debug, PartialEq, Eq)]
struct SnafuNumber(String);

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SnafuNumber> for usize {
    fn from(value: SnafuNumber) -> Self {
        value
            .0
            .bytes()
            .rev() // Only because we need inverted indices
            .enumerate()
            .map(|(index, ch)| {
                let value = match ch {
                    b'=' => -2,
                    b'-' => -1,
                    b'0' => 0,
                    b'1' => 1,
                    b'2' => 2,
                    _ => panic!("Invalid number"),
                };

                value * 5_isize.pow(index as u32)
            })
            .sum::<isize>()
            .try_into()
            .unwrap()
    }
}

impl From<usize> for SnafuNumber {
    fn from(value: usize) -> Self {
        // We start with the base 5 value, which is then changed as follows: if a digit is 3 or 4,
        // it needs to be replaced with a '=' or '-' instead, incrementing the preceding digit.
        let mut base5 = format!("{}", radix_5(value));
        let mut carry = 0;
        // SAFETY: Only working in ASCII.
        unsafe {
            for digit in base5.as_bytes_mut().iter_mut().rev() {
                let digit_value = *digit - b'0' + carry;
                *digit = match digit_value {
                    3 => {
                        carry = 1;
                        b'='
                    }
                    4 => {
                        carry = 1;
                        b'-'
                    }
                    5 => {
                        carry = 1;
                        b'0'
                    }
                    d => {
                        carry = 0;
                        d + b'0'
                    }
                };
            }

            if carry != 0 {
                base5.insert(0, '1');
            }

            Self(base5)
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(SnafuNumber::from(4890), part1(aoc_sample_input()));
    }

    #[test]
    fn test_snafu_to_usize() {
        assert_eq!(1, usize::from(SnafuNumber("1".into())));
        assert_eq!(2, usize::from(SnafuNumber("2".into())));
        assert_eq!(3, usize::from(SnafuNumber("1=".into())));
        assert_eq!(4, usize::from(SnafuNumber("1-".into())));
        assert_eq!(5, usize::from(SnafuNumber("10".into())));
        assert_eq!(6, usize::from(SnafuNumber("11".into())));
        assert_eq!(7, usize::from(SnafuNumber("12".into())));
        assert_eq!(8, usize::from(SnafuNumber("2=".into())));
        assert_eq!(9, usize::from(SnafuNumber("2-".into())));
        assert_eq!(10, usize::from(SnafuNumber("20".into())));
        assert_eq!(15, usize::from(SnafuNumber("1=0".into())));
        assert_eq!(20, usize::from(SnafuNumber("1-0".into())));
        assert_eq!(2022, usize::from(SnafuNumber("1=11-2".into())));
        assert_eq!(12345, usize::from(SnafuNumber("1-0---0".into())));
        assert_eq!(314159265, usize::from(SnafuNumber("1121-1110-1=0".into())));
    }

    #[test]
    fn test_usize_to_snafu() {
        assert_eq!(SnafuNumber::from(1), SnafuNumber("1".into()));
        assert_eq!(SnafuNumber::from(2), SnafuNumber("2".into()));
        assert_eq!(SnafuNumber::from(3), SnafuNumber("1=".into()));
        assert_eq!(SnafuNumber::from(4), SnafuNumber("1-".into()));
        assert_eq!(SnafuNumber::from(5), SnafuNumber("10".into()));
        assert_eq!(SnafuNumber::from(6), SnafuNumber("11".into()));
        assert_eq!(SnafuNumber::from(7), SnafuNumber("12".into()));
        assert_eq!(SnafuNumber::from(8), SnafuNumber("2=".into()));
        assert_eq!(SnafuNumber::from(9), SnafuNumber("2-".into()));
        assert_eq!(SnafuNumber::from(10), SnafuNumber("20".into()));
        assert_eq!(SnafuNumber::from(15), SnafuNumber("1=0".into()));
        assert_eq!(SnafuNumber::from(20), SnafuNumber("1-0".into()));
        assert_eq!(SnafuNumber::from(2022), SnafuNumber("1=11-2".into()));
        assert_eq!(SnafuNumber::from(12345), SnafuNumber("1-0---0".into()));
        assert_eq!(
            SnafuNumber::from(314159265),
            SnafuNumber("1121-1110-1=0".into())
        );
    }
}
