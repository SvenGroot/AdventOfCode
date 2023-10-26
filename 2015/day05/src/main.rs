// https://adventofcode.com/2015/day/5

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Determine how many strings are nice.
fn part1(input: AocInput) -> usize {
    input.filter(|s| is_nice(s)).count()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

fn is_nice(s: &str) -> bool {
    const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
    if s.bytes().filter(|ch| VOWELS.contains(ch)).count() < 3 {
        return false;
    }

    let mut has_double = false;
    if !s.as_bytes().windows(2).all(|window| {
        has_double = has_double || window[0] == window[1];
        window != [b'a', b'b']
            && window != [b'c', b'd']
            && window != [b'p', b'q']
            && window != [b'x', b'y']
    }) {
        return false;
    }

    has_double
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
