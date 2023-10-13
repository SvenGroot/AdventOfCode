// https://adventofcode.com/2021/day/10

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    input
        .filter_map(|s| find_illegal_character(s).map(get_score))
        .sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

fn find_illegal_character(s: String) -> Option<u8> {
    let mut expected = Vec::new();
    for ch in s.bytes() {
        let close = match ch {
            b'(' => b')',
            b'[' => b']',
            b'{' => b'}',
            b'<' => b'>',
            _ => {
                if let Some(expected) = expected.pop() {
                    if expected != ch {
                        return Some(ch);
                    }

                    continue;
                } else {
                    // Incomplete line
                    return None;
                }
            }
        };

        expected.push(close);
    }

    None
}

fn get_score(ch: u8) -> usize {
    match ch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(26397, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
