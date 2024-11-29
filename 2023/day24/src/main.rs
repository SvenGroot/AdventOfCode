// https://adventofcode.com/2023/day/24
// Requires nightly!
#![feature(f128)]

use std::{fmt::Display, str::FromStr};

use aoc::{
    grid3d::{GenericPoint3D, Point3D, PointDiff3D},
    input::AocInput,
    iterator::IteratorExt,
};

fn main() {
    println!(
        "Part 1: {}",
        part1(
            AocInput::from_input(),
            GenericPoint3D::<f128>::new(200000000000000.0, 200000000000000.0, 0.0),
            GenericPoint3D::<f128>::new(400000000000000.0, 400000000000000.0, 0.0)
        )
    );
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(
    input: AocInput,
    top_left: GenericPoint3D<f128>,
    bottom_right: GenericPoint3D<f128>,
) -> usize {
    let stones = input
        .map(|line| HailStone::from_str(&line).unwrap())
        .into_vec();

    let mut collisions = 0;
    for i in 0..stones.len() - 1 {
        for j in (i + 1)..stones.len() {
            let stone = &stones[i];
            let other = &stones[j];
            if let Some((x, y)) = stone.intersection2d(other) {
                if top_left.y() <= y
                    && top_left.x() <= x
                    && y <= bottom_right.y()
                    && x <= bottom_right.x()
                {
                    println!("Hailstone A: {stone}");
                    println!("Hailstone B: {other}");
                    println!("Intersection: {:?}", stone.intersection2d(other));
                    collisions += 1;
                }
            }
        }
    }

    collisions
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct HailStone {
    base: Point3D,
    vector: PointDiff3D,
}

impl HailStone {
    fn intersection2d(&self, other: &HailStone) -> Option<(f128, f128)> {
        let p1 = GenericPoint3D::new(self.base.x() as f128, self.base.y() as f128, 0.0);
        let mself = GenericPoint3D::new(self.vector.x() as f128, self.vector.y() as f128, 0.0);
        let p2 = p1 + mself;
        let mother = GenericPoint3D::new(other.vector.x() as f128, other.vector.y() as f128, 0.0);
        let p3: GenericPoint3D<f128> =
            GenericPoint3D::new(other.base.x() as f128, other.base.y() as f128, 0.0);
        let p4 = p3 + mother;

        let denominator =
            (p1.x() - p2.x()) * (p3.y() - p4.y()) - (p1.y() - p2.y()) * (p3.x() - p4.x());

        if denominator == 0.0 {
            return None;
        }

        let intersect_x = ((p1.x() * p2.y() - p1.y() * p2.x()) * (p3.x() - p4.x())
            - (p1.x() - p2.x()) * (p3.x() * p4.y() - p3.y() * p4.x()))
            / denominator;
        let intersect_y = ((p1.x() * p2.y() - p1.y() * p2.x()) * (p3.y() - p4.y())
            - (p1.y() - p2.y()) * (p3.x() * p4.y() - p3.y() * p4.x()))
            / denominator;

        // Check if the intersection is in the direction of the vector; if not, the collision is in
        // the past.
        let intersect = GenericPoint3D::new(intersect_x, intersect_y, 0.0);
        let diff = intersect - p1;
        if diff.x().signum() != mself.x().signum() || diff.y().signum() != mself.y().signum() {
            return None;
        }

        let diff = intersect - p3;
        if diff.x().signum() != mother.x().signum() || diff.y().signum() != mother.y().signum() {
            return None;
        }

        Some((intersect_x, intersect_y))
    }
}

impl FromStr for HailStone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (base, vector) = s.split_once(" @ ").unwrap();
        Ok(Self {
            base: base.parse().unwrap(),
            vector: vector.parse().unwrap(),
        })
    }
}

impl Display for HailStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.base, self.vector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            part1(
                AocInput::from_sample(),
                GenericPoint3D::<f128>::new(7.0, 7.0, 0.0),
                GenericPoint3D::<f128>::new(27.0, 27.0, 0.0)
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
