use aoc::get_input;
use std::str::FromStr;

fn main() {
    const PATH: &str = "input/day4.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn new(value: &str) -> Range {
        let (min, max) = value.split_once('-').unwrap();
        Self {
            min: u32::from_str(min).unwrap(),
            max: u32::from_str(max).unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        other.min >= self.min && other.max <= self.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.min <= other.max && other.min <= self.max
    }
}

fn part1(path: &str) -> u32 {
    get_input(path)
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let left = Range::new(left);
            let right = Range::new(right);
            u32::from(left.contains(&right) || right.contains(&left))
        })
        .sum()
}

fn part2(path: &str) -> u32 {
    get_input(path)
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let left = Range::new(left);
            let right = Range::new(right);
            u32::from(left.overlaps(&right))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "../input/sample/day4.txt";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(PATH));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(PATH));
    }
}
