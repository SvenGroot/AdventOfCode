// https://adventofcode.com/2021/day/19

use std::collections::HashMap;

use aoc::{grid3d::PointDiff3D, input::AocInput, slice::SliceCombinator};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    let mut scanners = Scanner::from_input(input);
    normalize_scanners(&mut scanners);
    0
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

fn normalize_scanners(scanners: &mut [Scanner]) {
    let mut seen = vec![false; scanners.len()];
    seen[0] = true;
    for x in 0..scanners.len() - 1 {
        for y in 0..scanners.len() {
            if seen[y] || x == y {
                continue;
            }

            if y > x {
                let (first, second) = scanners.split_at_mut(x + 1);
                if first[x].check_match(&mut second[y - x - 1]).is_some() {
                    seen[y] = true;
                }
            } else {
                let (first, second) = scanners.split_at_mut(y + 1);
                if second[x - y - 1].check_match(&mut first[y]).is_some() {
                    seen[y] = true;
                }
            }
        }
    }

    assert!(seen.iter().all(|v| *v));
}

#[derive(Clone)]
struct Scanner {
    beacons: Vec<PointDiff3D>,
}

impl Scanner {
    fn from_input(input: AocInput) -> Vec<Self> {
        let input = input.into_vec();
        input
            .split(|line| line.is_empty())
            .map(Self::from_slice)
            .collect()
    }

    fn from_slice(slice: &[String]) -> Self {
        let beacons: Vec<_> = slice[1..]
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();

        Self { beacons }
    }

    // /// Move all the beacons so they are positive from 0,0,0
    // fn normalize(&mut self) {
    //     let cube = DiffCube::from_points(self.beacons.iter());
    //     let offset = cube.top_left_front();
    //     self.mv(-offset);
    //     self.offset = Some(offset);
    // }

    // fn unnormalize(&mut self) {
    //     if let Some(offset) = self.offset.take() {
    //         self.mv(offset);
    //     }
    // }

    fn mv(&mut self, offset: PointDiff3D) {
        for beacon in &mut self.beacons {
            *beacon += offset;
        }
    }

    fn rotate_x(&mut self) {
        for beacon in &mut self.beacons {
            *beacon = beacon.rotate_x();
        }
    }

    fn rotate_y(&mut self) {
        for beacon in &mut self.beacons {
            *beacon = beacon.rotate_y();
        }
    }

    fn rotate_z(&mut self) {
        for beacon in &mut self.beacons {
            *beacon = beacon.rotate_z();
        }
    }

    fn check_match(&self, other: &mut Scanner) -> Option<PointDiff3D> {
        for _ in 0..4 {
            for _ in 0..4 {
                for _ in 0..4 {
                    if let Some(offset) = self.match_beacons(other) {
                        other.mv(offset);
                        return Some(offset);
                    }
                    other.rotate_z()
                }
                other.rotate_y()
            }
            other.rotate_x()
        }

        None
    }

    fn match_beacons(&self, other: &Scanner) -> Option<PointDiff3D> {
        let mut prev = None;
        let mut match_count = 0;
        let mut matches = HashMap::new();
        for (self1, self2) in self.beacons.as_slice().unordered_combinations() {
            if prev != Some(self1) {
                match_count = 0;
                matches.clear();
                prev = Some(self1);
            }

            let self_diff = *self2 - *self1;
            for (other1, other2) in other.beacons.as_slice().unordered_combinations() {
                let other_diff = *other2 - *other1;
                if other_diff == self_diff || other_diff == -self_diff {
                    match_count += 1;
                    for e in [*other1, *other2] {
                        matches
                            .entry(e)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }

            if match_count >= 11 {
                let matching_beacon = matches.iter().max_by_key(|x| x.1).unwrap().0;
                return Some(*self1 - *matching_beacon);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }

    #[test]
    fn test_check_match() {
        let mut scanners = Scanner::from_input(AocInput::from_sample());
        let (first, others) = scanners.split_at_mut(1);
        let offset = first[0].check_match(&mut others[0]).unwrap();
        assert_eq!(PointDiff3D::new(68, -1246, -43), offset);
    }
}
