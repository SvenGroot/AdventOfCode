// https://adventofcode.com/2022/day/%DAY%

use std::{path::Path, str::FromStr};

use aoc::{aoc_input, get_input};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let input: Vec<_> = get_input(path)
        .map(|line| usize::from_str(&line).unwrap())
        .collect();

    input.windows(2).filter(|slice| slice[1] > slice[0]).count()
}

fn part2(path: impl AsRef<Path>) -> usize {
    let input: Vec<_> = get_input(path)
        .map(|line| usize::from_str(&line).unwrap())
        .collect();

    let input: Vec<_> = input
        .windows(3)
        .map(|slice| slice.iter().sum::<usize>())
        .collect();

    input.windows(2).filter(|slice| slice[1] > slice[0]).count()
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5, part2(aoc_sample_input()));
    }
}
