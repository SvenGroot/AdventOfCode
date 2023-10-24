// https://adventofcode.com/2015/day/1

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> isize {
    parse_input(input).sum()
}

fn part2(input: AocInput) -> usize {
    let mut floor = 0;
    for (index, value) in parse_input(input).enumerate() {
        floor += value;
        if floor < 0 {
            return index + 1;
        }
    }

    unreachable!();
}

fn parse_input(input: AocInput) -> impl Iterator<Item = isize> {
    input
        .single_line()
        .into_bytes()
        .into_iter()
        .map(|ch| match ch {
            b'(' => 1,
            b')' => -1,
            _ => unreachable!(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(-3, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(AocInput::from_sample()));
    }
}
