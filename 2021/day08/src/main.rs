// https://adventofcode.com/2021/day/8

use std::str::FromStr;

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Count the number of 1, 4, 7, and 8 digits in the display outputs.
fn part1(input: AocInput) -> usize {
    input
        .parsed::<SegmentDisplay>()
        .map(|display| display.count_easy_digits())
        .sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct SegmentDisplay {
    _patterns: Vec<String>,
    output: Vec<String>,
}

impl SegmentDisplay {
    fn count_easy_digits(&self) -> usize {
        // 1, 4, 7 and 8 use a unique number of segments, so only look at that.
        self.output
            .iter()
            .filter(|digit| [2, 4, 3, 7].contains(&digit.len()))
            .count()
    }
}

impl FromStr for SegmentDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns, output) = s.split_once(" | ").unwrap();
        Ok(Self {
            _patterns: patterns
                .split_ascii_whitespace()
                .map(str::to_string)
                .collect(),
            output: output
                .split_ascii_whitespace()
                .map(str::to_string)
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(26, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
