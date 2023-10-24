// https://adventofcode.com/2015/day/2

use std::str::FromStr;

use aoc::{
    grid3d::{DiffCuboid, PointDiff3D},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// How much wrapping paper is needed for all the presents.
fn part1(input: AocInput) -> usize {
    input
        .parsed::<Present>()
        .map(|present| present.wrapping_needed())
        .sum()
}

// How much ribbon is needed.
fn part2(input: AocInput) -> usize {
    input
        .parsed::<Present>()
        .map(|present| present.ribbon_needed())
        .sum()
}

struct Present(DiffCuboid);

impl Present {
    fn wrapping_needed(&self) -> usize {
        let mut dimensions = [self.0.width(), self.0.height(), self.0.depth()];
        dimensions.sort();
        let slack = dimensions[0] * dimensions[1];
        self.0.surface_area() + slack
    }

    fn ribbon_needed(&self) -> usize {
        let mut dimensions = [self.0.width(), self.0.height(), self.0.depth()];
        dimensions.sort();
        let bow = 2 * (dimensions[0] + dimensions[1]);
        self.0.volume() + bow
    }
}

impl FromStr for Present {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('x');
        let depth = parts.next().unwrap().parse().unwrap();
        let width = parts.next().unwrap().parse().unwrap();
        let height = parts.next().unwrap().parse().unwrap();
        Ok(Self(DiffCuboid::from_size(
            PointDiff3D::default(),
            width,
            height,
            depth,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(101, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, part2(AocInput::from_sample()));
    }
}
