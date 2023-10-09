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
    let gamma_rate: usize = (0..len)
        .map(|bit_pos| get_most_common_bit(&input, bit_pos) << (len - bit_pos - 1))
        .sum();

    // Epsilon is just the bitwise not of gamma, only considering bits that are part of the number.
    let epsilon_rate = !gamma_rate & ((1 << len) - 1);
    println!("Gamma: {gamma_rate}, Epsilon: {epsilon_rate}");
    gamma_rate * epsilon_rate
}

// Find the ratings by eliminating numbers that don't match the most/least common bit at that index,
// for each index until there is only one number left.
fn part2(input: AocInput) -> usize {
    let input = input.into_vec();
    let o2_rating = find_rating(input.clone(), false);
    let co2_rating = find_rating(input.clone(), true);
    println!("Oxygen generator rating: {o2_rating}; CO2 scrubber rating: {co2_rating}");
    o2_rating * co2_rating
}

fn get_most_common_bit(input: &[String], bit_pos: usize) -> usize {
    let one_count = input
        .iter()
        .filter(|line| line.as_bytes()[bit_pos] == b'1')
        .count();

    let zero_count = input.len() - one_count;
    (one_count >= zero_count).into()
}

fn find_rating(mut values: Vec<String>, least_common: bool) -> usize {
    let mut bit_pos = 0;
    while values.len() > 1 {
        let mut compare_bit = get_most_common_bit(&values, bit_pos) as u8;
        // Invert the bit if least common is wanted.
        if least_common {
            compare_bit = (compare_bit == 0).into();
        }

        compare_bit += b'0';
        values.retain(|value| value.as_bytes()[bit_pos] == compare_bit);
        bit_pos += 1;
    }

    usize::from_str_radix(&values[0], 2).unwrap()
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
        assert_eq!(230, part2(AocInput::from_sample()));
    }
}
