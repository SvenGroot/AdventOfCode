// https://adventofcode.com/2022/day/1
use std::str::FromStr;

use aoc::get_input_vec;

fn main() {
    let mut result = get_input_vec("input/day1.txt")
        .split(|l| l.is_empty())
        .map(|split| split.iter().map(|l| u32::from_str(l).unwrap()).sum())
        .collect::<Vec<u32>>();

    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    println!("Sum: {}", sum);
}
