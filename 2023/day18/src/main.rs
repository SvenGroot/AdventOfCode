// https://adventofcode.com/2023/day/18

use std::str::FromStr;

use aoc::{grid::PointDiff, input::AocInput};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Calculate the trench size.
fn part1(input: AocInput) -> usize {
    let map = DigMap::from_input(input);
    map.trench_size()
}

// Calculate the trench size, using instructions decoded from the "color" field.
fn part2(input: AocInput) -> usize {
    let map = DigMap::from_fixed_input(input);
    map.trench_size()
}

struct DigMap(Vec<PointDiff>, usize);

impl DigMap {
    fn from_input(input: AocInput) -> Self {
        let mut current = PointDiff::default();
        let mut result = vec![current];
        let mut length = 0;
        for instruction in input.parsed::<DigInstruction>() {
            current += instruction.dir * instruction.length as isize;
            length += instruction.length;
            result.push(current);
        }

        Self(result, length)
    }

    fn from_fixed_input(input: AocInput) -> Self {
        let mut current = PointDiff::default();
        let mut result = vec![current];
        let mut length = 0;
        for instruction in input.parsed::<FixedInstruction>() {
            current += instruction.dir * instruction.length as isize;
            length += instruction.length;
            result.push(current);
        }

        Self(result, length)
    }

    fn trench_size(&self) -> usize {
        let sum1: isize = self
            .0
            .windows(2)
            .map(|window| window[0].row() * window[1].col())
            .sum();

        let sum2: isize = self
            .0
            .windows(2)
            .map(|window| window[0].col() * window[1].row())
            .sum();

        // I found this method of getting the area online, but the formula given is just
        // abs(sum1 - sum2) / 2. For some reason we need to add half the perimeter as well, and then
        // it's off by one. Not sure why, but it works.
        (sum1.abs_diff(sum2) + self.1) / 2 + 1
    }
}

struct DigInstruction {
    dir: PointDiff,
    length: usize,
}

impl FromStr for DigInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let dir = parts.next().unwrap();
        let dir = PointDiff::from_char(dir.as_bytes()[0], [b'U', b'R', b'D', b'L']).unwrap();
        let length = parts.next().unwrap().parse().unwrap();
        Ok(Self { dir, length })
    }
}

struct FixedInstruction {
    dir: PointDiff,
    length: usize,
}

impl FromStr for FixedInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = s.split(' ').nth(2).unwrap();
        let length = &color[2..7];
        let dir = color.as_bytes()[7];
        Ok(Self {
            dir: PointDiff::from_char(dir, [b'3', b'0', b'1', b'2']).unwrap(),
            length: usize::from_str_radix(length, 16).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(62, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(952408144115, part2(AocInput::from_sample()));
    }
}
