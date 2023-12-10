// https://adventofcode.com/2023/day/9

use std::str::FromStr;

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find the next value in every sequence.
fn part1(input: AocInput) -> isize {
    input
        .parsed::<Sequence>()
        .map(|s| Sequence::extend(&s))
        .sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[derive(Clone)]
struct Sequence(Vec<isize>);

impl Sequence {
    fn diff(&self) -> Self {
        Self(
            self.0
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        )
    }

    fn extend(&self) -> isize {
        let diffs = self.diffs();
        let mut target = 0;
        for diff in diffs {
            target += diff.0.last().unwrap();
        }

        target
    }

    fn diffs(&self) -> Vec<Self> {
        let mut result = vec![self.clone()];
        loop {
            let current = result.last().unwrap();
            let diff = current.diff();
            if diff.all_zeroes() {
                break;
            }

            result.push(diff);
        }

        result
    }

    fn all_zeroes(&self) -> bool {
        self.0.iter().all(|value| *value == 0)
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(' ').map(|value| value.parse().unwrap()).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(114, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
