// https://adventofcode.com/2015/day/4

use aoc::input::AocInput;
use md5::{Digest, Md5};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find a hash that starts with five zeroes.
fn part1(input: AocInput) -> usize {
    let input = input.single_line().into_bytes();
    let mut hasher = Md5::new();
    hasher.update(&input);
    for i in 0..100000000 {
        if test_hash(hasher.clone(), i) {
            return i;
        }
    }

    panic!();
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

fn test_hash(mut base: Md5, suffix: usize) -> bool {
    let suffix = suffix.to_string().into_bytes();
    base.update(&suffix);
    let digest = base.finalize();
    digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(609043, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
