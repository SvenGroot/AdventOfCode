// https://adventofcode.com/2024/day/3

use aoc::input::AocInput;
use regex::Regex;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Sum the result of all the mul() instructions.
fn part1(input: AocInput) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    input
        .map(|line| {
            re.captures_iter(&line)
                .map(|c| {
                    let (_, [left, right]) = c.extract();
                    left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap()
                })
                .sum::<usize>()
        })
        .sum()
}

// Same, but also use do() and don't() instructions to toggle processing mul() on/off.
fn part2(input: AocInput) -> usize {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut sum = 0;
    let mut active = true;
    for line in input {
        for c in re.captures_iter(&line) {
            let m = c.get(0).unwrap().as_str();
            match m {
                "do()" => active = true,
                "don't()" => active = false,
                _ => {
                    if active {
                        let left = c.get(1).unwrap().as_str();
                        let right = c.get(2).unwrap().as_str();
                        sum += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap()
                    }
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            48,
            part2(AocInput::from_file(AocInput::get_custom_path(
                "day3_part2.txt",
                true
            )))
        );
    }
}
