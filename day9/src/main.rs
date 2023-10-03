use aoc::{
    aoc_input, get_input,
    grid::{Point, PointDiff},
};
use std::{collections::HashSet, path::Path, str::FromStr};

fn main() {
    println!("Part 1: {}", part1(aoc_input()));
    println!("Part 2: {}", part2(aoc_input()));
}

fn part1(path: impl AsRef<Path>) -> usize {
    simulate(path, 2)
}

fn part2(path: impl AsRef<Path>) -> usize {
    simulate(path, 10)
}

fn simulate(path: impl AsRef<Path>, knots: usize) -> usize {
    let mut visited = HashSet::new();
    let mut knots = vec![Point::new(1000, 1000); knots];
    visited.insert(*knots.last().unwrap());
    for line in get_input(path) {
        let amount = i32::from_str(&line[2..]).unwrap();
        for _ in 0..amount {
            let dir = get_direction(line.as_bytes()[0]);
            knots[0] += dir;
            for i in 1..knots.len() {
                if !knots[i].is_adjacent(knots[i - 1]) {
                    let dir = knots[i - 1].diff(knots[i]).unwrap().signum();
                    knots[i] += dir;
                    visited.insert(*knots.last().unwrap());
                }
            }
        }
    }

    visited.len()
}

fn get_direction(dir: u8) -> PointDiff {
    PointDiff::from_char(dir, b'U', b'R', b'D', b'L').unwrap()
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(36, part2(aoc_sample_input().with_file_name("day9_2.txt")));
    }
}
