// https://adventofcode.com/2021/day/8

use std::{collections::HashSet, str::FromStr};

use aoc::{bitfield::BitField, input::AocInput};

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

// Sum the output of each display.
fn part2(input: AocInput) -> usize {
    input
        .parsed::<SegmentDisplay>()
        .map(|display| display.get_output())
        .sum()
}

struct DisplayPattern(BitField);

impl FromStr for DisplayPattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = BitField::default();
        for ch in s.bytes() {
            let index = ch - b'a';
            assert!(index < 7);
            result = result.set(index as usize, true);
        }

        Ok(DisplayPattern(result))
    }
}

struct SegmentDisplay {
    patterns: Vec<DisplayPattern>,
    output: Vec<DisplayPattern>,
}

impl SegmentDisplay {
    fn count_easy_digits(&self) -> usize {
        // 1, 4, 7 and 8 use a unique number of segments, so only look at that.
        self.output
            .iter()
            .filter(|digit| [2, 4, 3, 7].contains(&digit.0.one_bits()))
            .count()
    }

    fn get_output(&self) -> usize {
        let mut unknown: HashSet<_> = self.patterns.iter().map(|p| p.0).collect();
        let mut known = [None; 10];

        // Step 0: determine the easy numbers
        for pattern in &unknown {
            let index = match pattern.one_bits() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                _ => continue,
            };

            assert!(known[index].replace(*pattern).is_none());
        }

        unknown.retain(|p| ![2, 4, 3, 7].contains(&p.one_bits()));

        // Step 1: digit 6 is a pattern of length 6 missing either segment used in 1.
        let one = known[1].unwrap();
        let six = Self::find_digit(&mut unknown, &mut known, 6, |p| {
            p.one_bits() == 6 && !p.test_all_bits(one)
        });

        // Step 2: digit 5 is length 5, missing a.
        let segment_b = six & one;
        let segment_a = one & !segment_b;
        let five = Self::find_digit(&mut unknown, &mut known, 5, |p| {
            p.one_bits() == 5 && !p.test_all_bits(segment_a)
        });

        // Step 3: digit 2 is length 5, missing b.
        Self::find_digit(&mut unknown, &mut known, 2, |p| {
            p.one_bits() == 5 && !p.test_all_bits(segment_b)
        });

        // Step 4: digit 3 is the remaining length 5.
        Self::find_digit(&mut unknown, &mut known, 3, |p| p.one_bits() == 5);

        // Step 5: digit 9 is missing segment e
        let segment_e = six & !five;
        Self::find_digit(&mut unknown, &mut known, 9, |p| !p.test_all_bits(segment_e));

        // Step 6: digit 0 is the one remaining
        assert!(unknown.len() == 1);
        known[0] = Some(*unknown.iter().next().unwrap());

        // Now determine the code.
        let mut result = 0;
        for (index, digit) in self.output.iter().rev().enumerate() {
            let digit = known.iter().position(|p| p.unwrap() == digit.0).unwrap();
            result += digit * 10usize.pow(index as u32);
        }

        result
    }

    fn find_digit(
        unknown: &mut HashSet<BitField>,
        known: &mut [Option<BitField>],
        digit: usize,
        pred: impl Fn(BitField) -> bool,
    ) -> BitField {
        let pattern = *unknown.iter().find(|p| pred(**p)).unwrap();

        known[digit] = Some(pattern);
        unknown.remove(&pattern);
        pattern
    }
}

impl FromStr for SegmentDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns, output) = s.split_once(" | ").unwrap();
        Ok(Self {
            patterns: patterns
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            output: output
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
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
        assert_eq!(61229, part2(AocInput::from_sample()));
    }

    #[test]
    fn test_output() {
        let display = SegmentDisplay::from_str(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();

        assert_eq!(5353, display.get_output())
    }
}
