// https://adventofcode.com/2021/day/10

use std::str::FromStr;

use aoc::{input::AocInput, iterator::IntoVec};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    input
        .parsed::<ParsedLine>()
        .filter_map(|line| line.get_illegal_character().map(get_score))
        .sum()
}

fn part2(input: AocInput) -> usize {
    let mut scores = input
        .parsed::<ParsedLine>()
        .filter_map(|line| line.get_autocomplete_score())
        .into_vec();

    scores.sort();
    scores[(scores.len() - 1) / 2]
}

enum ParsedLine {
    Corrupt(u8),
    Incomplete(Vec<u8>),
}

impl ParsedLine {
    fn get_illegal_character(&self) -> Option<u8> {
        match self {
            ParsedLine::Corrupt(ch) => Some(*ch),
            _ => None,
        }
    }

    fn get_autocomplete_score(self) -> Option<usize> {
        let ParsedLine::Incomplete(mut expected) = self else {
            return None;
        };

        let mut score = 0;
        while let Some(next) = expected.pop() {
            let char_score = match next {
                b')' => 1,
                b']' => 2,
                b'}' => 3,
                b'>' => 4,
                _ => unreachable!(),
            };

            score = score * 5 + char_score;
        }

        Some(score)
    }
}

impl FromStr for ParsedLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut expected = Vec::new();
        for ch in s.bytes() {
            let close = match ch {
                b'(' => b')',
                b'[' => b']',
                b'{' => b'}',
                b'<' => b'>',
                _ => {
                    if let Some(expected) = expected.pop() {
                        if expected != ch {
                            return Ok(ParsedLine::Corrupt(ch));
                        }

                        continue;
                    } else {
                        return Ok(ParsedLine::Incomplete(expected));
                    }
                }
            };

            expected.push(close);
        }

        Ok(ParsedLine::Incomplete(expected))
    }
}

fn get_score(ch: u8) -> usize {
    match ch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(26397, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(288957, part2(AocInput::from_sample()));
    }
}
