// https://adventofcode.com/2022/day/1
use std::{path::Path, str::FromStr};

use aoc::{aoc_input, get_input_vec};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> u32 {
    get_input_vec(path)
        .split(|l| l.is_empty())
        .map(|split| split.iter().map(|l| u32::from_str(l).unwrap()).sum())
        .max()
        .unwrap()
}

fn part2(path: impl AsRef<Path>) -> u32 {
    let mut result = get_input_vec(path)
        .split(|l| l.is_empty())
        .map(|split| split.iter().map(|l| u32::from_str(l).unwrap()).sum())
        .collect::<Vec<u32>>();

    result.sort_by(|a, b| b.cmp(a));
    result.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(24000, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45000, part2(aoc_sample_input()));
    }
}
