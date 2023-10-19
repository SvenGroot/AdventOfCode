// https://adventofcode.com/2021/day/22

use std::{collections::HashSet, str::FromStr};

use aoc::{
    grid3d::{DiffCube, PointDiff3D},
    input::AocInput,
};
use text_io::scan;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Check how many cubes are on in the initialization region.
// N.B. I know full well this method won't work for part 2.
fn part1(input: AocInput) -> usize {
    let mut cubes = HashSet::new();
    let init_region = DiffCube::new(
        PointDiff3D::new(-50, -50, -50),
        PointDiff3D::new(50, 50, 50),
    );

    for step in input
        .parsed::<RebootStep>()
        .filter(|step| init_region.contains_cube(&step.cube))
    {
        step.process(&mut cubes);
    }

    cubes.len()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct RebootStep {
    on: bool,
    cube: DiffCube,
}

impl RebootStep {
    fn process(&self, cubes: &mut HashSet<PointDiff3D>) {
        for pos in self.cube.points() {
            if self.on {
                cubes.insert(pos);
            } else {
                cubes.remove(&pos);
            }
        }
    }
}

impl FromStr for RebootStep {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').unwrap();
        let on = left == "on";
        let min_x: isize;
        let max_x: isize;
        let min_y: isize;
        let max_y: isize;
        let min_z: isize;
        let max_z: isize;
        scan!(right.bytes() => "x={}..{},y={}..{},z={}..{}", min_x, max_x, min_y, max_y, min_z, max_z);
        let top_left_front = PointDiff3D::new(min_x, min_y, min_z);
        let bottom_right_back = PointDiff3D::new(max_x, max_y, max_z);
        let cube = DiffCube::new(top_left_front, bottom_right_back);
        Ok(Self { on, cube })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(590784, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
