#![allow(unused_variables)]
#![allow(unused_imports)]

mod stacks;
#[macro_use]
extern crate text_io;

use aoc::get_input;
use aoc::get_input_vec;
use stacks::Stacks;
use std::str::FromStr;

fn main() {
    const PATH: &str = "input/day5.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

fn part1(path: &str) -> String {
    move_crates(path, false)
}

fn part2(path: &str) -> String {
    move_crates(path, true)
}

fn move_crates(path: &str, multi: bool) -> String {
    let input = get_input_vec(path);
    let mut splits = input.split(|l| l.is_empty());
    let mut stacks = Stacks::new(splits.next().unwrap());
    for op in splits.next().unwrap() {
        let (from, to, count) = parse_op(op);
        if multi {
            stacks.mv_multi(from - 1, to - 1, count);
        } else {
            stacks.mv(from - 1, to - 1, count);
        }
    }

    println!("{}", stacks);

    stacks.tops().collect()
}

fn parse_op(op: &str) -> (usize, usize, usize) {
    let from: usize;
    let to: usize;
    let count: usize;

    scan!(op.bytes()  => "move {} from {} to {}", count, from, to);
    (from, to, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "../input/sample/day5.txt";

    #[test]
    fn test_part1() {
        assert_eq!("CMZ", part1(PATH));
    }

    #[test]
    fn test_part2() {
        assert_eq!("MCD", part2(PATH));
    }
}
