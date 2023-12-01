// https://adventofcode.com/2023/day/1

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Construct a two-digit value from the first digit in the string and the last digit in the string.
fn part1(input: AocInput) -> usize {
    input
        .map(|line| first_digit(&line) * 10 + last_digit(&line))
        .sum()
}

// Same, but spelled-out digits also count.
fn part2(input: AocInput) -> usize {
    input
        .map(|line| find_string_digit(&line, false) * 10 + find_string_digit(&line, true))
        .sum()
}

fn first_digit(s: &str) -> usize {
    s.chars().find(char::is_ascii_digit).unwrap() as usize - b'0' as usize
}

fn last_digit(s: &str) -> usize {
    s.chars().rev().find(char::is_ascii_digit).unwrap() as usize - b'0' as usize
}

fn find_string_digit(s: &str, reverse: bool) -> usize {
    const DIGITS: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    // Find the digit with the lowest (or highest, if reverse) index in the string.
    let result = DIGITS.iter().enumerate().filter_map(|(digit_index, &d)| {
        (if reverse { s.rfind(d) } else { s.find(d) })
            .map(|string_index| (string_index, digit_index))
    });

    let result = if reverse {
        result.max_by_key(|value| value.0)
    } else {
        result.min_by_key(|value| value.0)
    };

    result.unwrap().1 % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(142, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            281,
            part2(AocInput::from_file(AocInput::get_custom_path(
                "day1_part2",
                true
            )))
        );
    }
}
