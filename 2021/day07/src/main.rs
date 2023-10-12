// https://adventofcode.com/2021/day/7

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Get the minimum fuel required for all crabs to line up at the same position.
// Linear fuel usage.
fn part1(input: AocInput) -> isize {
    calculate_min_fuel(input, |x, crab| (crab - x).abs())
}

// Same, but increasing fuel usage over distance.
fn part2(input: AocInput) -> isize {
    calculate_min_fuel(input, |x, crab| {
        let dist = (crab - x).abs();
        // Calculate 1 + 2 + .. + dist
        dist * (dist + 1) / 2
    })
}

fn calculate_min_fuel(input: AocInput, f: impl Fn(isize, isize) -> isize) -> isize {
    let crabs = input.single_line_parsed::<isize>(',');
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    // Get the position with the lowest sum of fuel used for each crab.
    (*min..=*max)
        .map(|x| crabs.iter().map(|crab| f(x, *crab)).sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(37, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(168, part2(AocInput::from_sample()));
    }
}
