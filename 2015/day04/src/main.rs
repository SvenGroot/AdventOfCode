// https://adventofcode.com/2015/day/4

use aoc::input::AocInput;
use md5::{digest::Output, Digest, Md5};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find a hash that starts with five zeroes.
fn part1(input: AocInput) -> usize {
    find_hash(input, |digest| {
        digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10
    })
}

// Find a hash that starts with six zeroes.
fn part2(input: AocInput) -> usize {
    find_hash(input, |digest| {
        digest[0] == 0 && digest[1] == 0 && digest[2] == 0
    })
}

fn find_hash(input: AocInput, pred: impl Fn(Output<Md5>) -> bool) -> usize {
    let input = input.single_line().into_bytes();
    let mut hasher = Md5::new();
    hasher.update(&input);
    for i in 0..100000000 {
        if pred(test_hash(hasher.clone(), i)) {
            return i;
        }
    }

    panic!();
}

fn test_hash(mut base: Md5, suffix: usize) -> Output<Md5> {
    let suffix = suffix.to_string().into_bytes();
    base.update(&suffix);
    base.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(609043, part1(AocInput::from_sample()));
    }
}
