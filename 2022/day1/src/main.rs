// https://adventofcode.com/2022/day/1

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> u32 {
    input
        .parsed_opt::<u32>()
        .into_vec()
        .split(Option::is_none)
        .map(|split| split.iter().map(|value| value.unwrap()).sum())
        .max()
        .unwrap()
}

fn part2(input: AocInput) -> u32 {
    let mut result: Vec<u32> = input
        .parsed_opt::<u32>()
        .into_vec()
        .split(Option::is_none)
        .map(|split| split.iter().map(|value: &Option<u32>| value.unwrap()).sum())
        .collect();

    result.sort_by(|a, b| b.cmp(a));
    result.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(24000, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45000, part2(AocInput::from_sample()));
    }
}
