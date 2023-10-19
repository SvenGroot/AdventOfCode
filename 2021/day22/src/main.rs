// https://adventofcode.com/2021/day/22

use std::str::FromStr;

use aoc::{
    grid3d::{DiffCuboid, PointDiff3D},
    input::AocInput,
};
use text_io::scan;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Check how many cubes are on in the initialization region.
fn part1(input: AocInput) -> usize {
    process_steps(input, true)
}

// Check how many cubes are on in all regions.
fn part2(input: AocInput) -> usize {
    process_steps(input, false)
}

fn process_steps(input: AocInput, filter: bool) -> usize {
    let mut cubes = Vec::new();
    let init_region = DiffCuboid::new(
        PointDiff3D::new(-50, -50, -50),
        PointDiff3D::new(50, 50, 50),
    );

    let mut steps: Box<dyn Iterator<Item = RebootStep>> = Box::new(input.parsed::<RebootStep>());
    if filter {
        steps = Box::new(steps.filter(|step| init_region.contains_cube(&step.cube)));
    }

    for step in steps {
        cubes = step.process(cubes);
        println!(
            "current: {}, {}",
            step.on,
            cubes.iter().map(DiffCuboid::volume).sum::<usize>()
        )
    }

    cubes.iter().map(DiffCuboid::volume).sum()
}

struct RebootStep {
    on: bool,
    cube: DiffCuboid,
}

impl RebootStep {
    fn process(&self, cubes: Vec<DiffCuboid>) -> Vec<DiffCuboid> {
        let mut new_cubes = Vec::new();
        for cube in cubes {
            let mut split = cube.diff(&self.cube);
            new_cubes.append(&mut split);
        }

        if self.on {
            new_cubes.push(self.cube);
        }

        new_cubes
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
        let cube = DiffCuboid::new(top_left_front, bottom_right_back);
        Ok(Self { on, cube })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(474140, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2758514936282235, part2(AocInput::from_sample()));
    }
}
