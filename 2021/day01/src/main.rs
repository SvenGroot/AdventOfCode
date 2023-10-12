// https://adventofcode.com/2021/day/1

use aoc::{input::AocInput, iterator::IntoVec};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    input
        .parsed::<usize>()
        .into_vec()
        .windows(2)
        .filter(|slice| slice[1] > slice[0])
        .count()
}

fn part2(input: AocInput) -> usize {
    let input: Vec<_> = input
        .parsed::<usize>()
        .into_vec()
        .windows(3)
        .map(|slice| slice.iter().sum::<usize>())
        .collect();

    input.windows(2).filter(|slice| slice[1] > slice[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5, part2(AocInput::from_sample()));
    }
}
