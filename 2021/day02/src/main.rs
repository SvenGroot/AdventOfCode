// https://adventofcode.com/2021/day/2

use std::str::FromStr;

use aoc::{grid::PointDiff, input::AocInput};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Get the product of the x and y coordinates of the final position.
fn part1(input: AocInput) -> isize {
    let pos: PointDiff = input.parsed::<Command>().map(|c| c.0 * c.1).sum();
    pos.row() * pos.col()
}

// Get the product of the x and y coordinates of the final position using the alternate rules.
fn part2(input: AocInput) -> isize {
    let mut pos = PointDiff::default();
    let mut aim = 0;
    for command in input.parsed::<Command>() {
        match command.0 {
            PointDiff::DOWN => aim += command.1,
            PointDiff::UP => aim -= command.1,
            PointDiff::RIGHT => {
                pos += PointDiff::RIGHT * command.1;
                pos += PointDiff::DOWN * aim * command.1;
            }
            _ => unreachable!(),
        }
    }

    pos.row() * pos.col()
}

struct Command(PointDiff, isize);

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, scale) = s.split_once(' ').unwrap();
        let dir = PointDiff::from_str(dir, ["up", "forward", "down", ""]).unwrap();
        assert!(dir != PointDiff::LEFT);
        let scale = scale.parse().unwrap();
        Ok(Command(dir, scale))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(150, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(900, part2(AocInput::from_sample()));
    }
}
