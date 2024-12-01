// https://adventofcode.com/2024/day/1

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Get the distance between each number in the two lists after sorting them.
fn part1(input: AocInput) -> usize {
    let (mut list1, mut list2) = parse_lists(input);
    list1.sort();
    list2.sort();
    list1
        .into_iter()
        .zip(list2)
        .map(|(left, right)| (right - left).unsigned_abs())
        .sum()
}

// Multiply each number in list 1 with how often it occurs in list 2.
fn part2(input: AocInput) -> usize {
    let (list1, list2) = parse_lists(input);
    list1
        .into_iter()
        .map(|num1| num1 as usize * list2.iter().filter(|num2| num1 == **num2).count())
        .sum()
}

fn parse_lists(input: AocInput) -> (Vec<isize>, Vec<isize>) {
    input
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (
                left.parse::<isize>().unwrap(),
                right.parse::<isize>().unwrap(),
            )
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, part2(AocInput::from_sample()));
    }
}
