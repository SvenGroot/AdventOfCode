// https://adventofcode.com/2022/day/6
#![allow(unused_variables)]
#![allow(unused_imports)]

use aoc::sliding_window::HasSlidingWindow;
use aoc::{get_input, get_input_single};
use std::{collections::HashSet, str::FromStr};

fn main() {
    const PATH: &str = "input/day6.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

fn part1(path: &str) -> usize {
    find_marker(&get_input_single(path), 4)
}

fn part2(path: &str) -> usize {
    find_marker(&get_input_single(path), 14)
}

fn find_marker(text: &str, window_size: usize) -> usize {
    for (index, window) in text.as_bytes().sliding_window(window_size).enumerate() {
        if has_no_duplicates(window) {
            return index + window_size;
        }
    }

    panic!("No marker");
}

fn has_no_duplicates(slice: &[u8]) -> bool {
    let mut unique = HashSet::new();
    slice.iter().all(move |item| unique.insert(*item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_marker() {
        assert_eq!(7, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
        assert_eq!(19, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    }
}
