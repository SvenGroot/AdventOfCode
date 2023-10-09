// https://adventofcode.com/2021/day/3

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Determine gamma rate * epsilon rate, where:
// gamma rate: each bit is the most common bit at that index.
// epsilon rate: each bit is the least common bit at that index.
fn part1(input: AocInput) -> usize {
    let input = input.into_vec();
    let len = input[0].len();
    let count = input.len();
    let gamma_rate: usize = (0..len)
        .map(|index| {
            if input
                .iter()
                .filter(|line| line.as_bytes()[index] == b'1')
                .count()
                > count / 2
            {
                1 << (len - index - 1)
            } else {
                0
            }
        })
        .sum();

    // Epsilon is just the bitwise not of gamma, only considering bits that are part of the number.
    let epsilon_rate = !gamma_rate & ((1 << len) - 1);
    println!("Gamma: {gamma_rate}, Epsilon: {epsilon_rate}");
    gamma_rate * epsilon_rate
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(198, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
