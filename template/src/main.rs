#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use aoc::get_input;
use std::str::FromStr;

fn main() {
    const PATH: &str = "input/day4.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

fn part1(path: &str) -> u32 {
    get_input(path).map(|line| 0).sum()
}

fn part2(path: &str) -> u32 {
    get_input(path).map(|line| 0).sum()
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
