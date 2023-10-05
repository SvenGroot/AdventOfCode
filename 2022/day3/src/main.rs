// https://adventofcode.com/2022/day/3
// I've decided to make things easier on myself and just unwrap stuff instead of dealing with errors
// properly.
use aoc::{get_input, get_input_vec};

fn main() {
    part1("input/day3.txt");
    part2("input/day3.txt");
}

fn part1(path: &str) -> u32 {
    let sum: u32 = get_input(path)
        .map(|line| {
            let left = &line[0..line.len() / 2];
            let right = &line[line.len() / 2..];
            let common = left.chars().find(|ch| right.contains(*ch)).unwrap();
            get_priority(common)
        })
        .sum();

    println!("Sum: {}", sum);
    sum
}

fn part2(path: &str) -> u32 {
    let sum: u32 = get_input_vec(path)
        .chunks(3)
        .map(|group| {
            let common = group[0]
                .chars()
                .find(|ch| group[1..].iter().all(|g| g.contains(*ch)))
                .unwrap();
            get_priority(common)
        })
        .sum();

    println!("Sum: {}", sum);
    sum
}

fn get_priority(ch: char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => panic!("Invalid input."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(157, part1("../input/sample/day3.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(70, part2("../input/sample/day3.txt"));
    }
}
