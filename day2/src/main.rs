// https://adventofcode.com/2022/day/2

use std::path::Path;

use aoc::{aoc_input, get_input};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> i32 {
    get_input(path)
        .map(|line| {
            let bytes = line.as_bytes();
            let opponent_choice = bytes[0] as i32 - 'A' as i32;
            let my_choice = bytes[2] as i32 - 'X' as i32;
            let outcome = (my_choice - opponent_choice).rem_euclid(3);
            let points = match outcome {
                1 => 6,
                0 => 3,
                _ => 0,
            };

            points + my_choice + 1
        })
        .sum()
}

fn part2(path: impl AsRef<Path>) -> i32 {
    get_input(path)
        .map(|line| {
            let opponent_choice = line.as_bytes()[0] as i32 - 'A' as i32;
            let outcome = line.as_bytes()[2] as i32 - 'Y' as i32;
            let my_choice = (opponent_choice + outcome).rem_euclid(3);
            my_choice
                + 1
                + match outcome {
                    1 => 6,
                    0 => 3,
                    _ => 0,
                }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(15, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2(aoc_sample_input()));
    }
}
