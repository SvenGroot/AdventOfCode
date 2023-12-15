// https://adventofcode.com/2023/day/15

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Apply the HASH algorithm to every step in the sequence.
fn part1(input: AocInput) -> u64 {
    input.single_line().split(',').map(hash_str).sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

fn hash_str(s: &str) -> u64 {
    s.bytes()
        .fold(0, |current, b| ((current + b as u64) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1320, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
