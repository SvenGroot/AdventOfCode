// https://adventofcode.com/2015/day/3

use std::collections::HashSet;

use aoc::{grid::PointDiff, input::AocInput};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Count unique houses visited by santa.
fn part1(input: AocInput) -> usize {
    get_houses(&input.single_line())
}

// Count unique houses visited by santa and robo-santa.
fn part2(input: AocInput) -> usize {
    get_houses_part2(&input.single_line())
}

fn get_houses(directions: &str) -> usize {
    let houses = visit_houses(parse_input(directions));
    count_houses(houses)
}

fn get_houses_part2(directions: &str) -> usize {
    let directions = parse_input(directions);

    let santa = directions.clone().step_by(2);
    let robo_santa = directions.skip(1).step_by(2);

    let houses = visit_houses(santa).chain(visit_houses(robo_santa));

    count_houses(houses)
}

fn parse_input(directions: &str) -> impl Iterator<Item = PointDiff> + Clone + '_ {
    directions
        .bytes()
        .map(|ch| PointDiff::from_arrows(ch).unwrap())
}

fn visit_houses(directions: impl Iterator<Item = PointDiff>) -> impl Iterator<Item = PointDiff> {
    directions.scan(PointDiff::default(), |current, new| {
        *current += new;
        Some(*current)
    })
}

fn count_houses(houses: impl Iterator<Item = PointDiff>) -> usize {
    let mut set: HashSet<_> = houses.collect();

    // Include starting point.
    set.insert(PointDiff::default());
    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2, get_houses(">"));
        assert_eq!(4, get_houses("^>v<"));
        assert_eq!(2, get_houses("^v^v^v^v^v"))
    }

    #[test]
    fn test_part2() {
        assert_eq!(3, get_houses_part2("^v"));
        assert_eq!(3, get_houses_part2("^>v<"));
        assert_eq!(11, get_houses_part2("^v^v^v^v^v"))
    }
}
