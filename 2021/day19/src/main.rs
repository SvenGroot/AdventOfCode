// https://adventofcode.com/2021/day/19

use std::collections::{HashMap, HashSet};

use aoc::{grid3d::PointDiff3D, input::AocInput, slice::SliceExt};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    let mut scanners = Scanner::from_input(input);
    normalize_scanners(&mut scanners);
    get_all_beacons(&scanners).len()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

fn normalize_scanners(scanners: &mut [Scanner]) {
    // Normalized contains indices of scanners that are already moved to be from the perspective of
    // scanner 0, but have not yet been used to normalize other scanners.
    let mut normalized = vec![0];
    // Scanners that haven't been normalized yet.
    let mut remaining: Vec<_> = (1..scanners.len()).collect();

    while !remaining.is_empty() {
        let current = normalized.pop().unwrap();
        let mut new_remaining = Vec::new();
        for index in &remaining {
            let (current, other) = scanners.get_two_mut(current, *index);
            if let Some(offset) = current.check_match(other) {
                println!("Scanner {index} is at {offset}");
                normalized.push(*index);
            } else {
                new_remaining.push(*index);
            }
        }

        remaining = new_remaining
    }
}

fn get_all_beacons(scanners: &[Scanner]) -> HashSet<PointDiff3D> {
    scanners
        .iter()
        .flat_map(|scanner| scanner.beacons.iter())
        .copied()
        .collect()
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

    // Checks all possible rotations of other to see if it matches with self.
    //
    // If a match is found, other will be rotated and moved to have the same orientation and origin
    // as self. If not, it will be rotated fully back to its starting orientation.
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

        // To check if there's a match, we need to see if there are twelve beacons whose relative
        // positions are the same. To do this, we check every combination of beacons from self
        // against every combination of beacons from other.
        for (self1, self2) in self.beacons.unordered_combinations() {
            // self1 will only change if all values of self2 have been processed. Use this to check
            // if we found enough matches from self1.
            if prev != Some(self1) {
                match_count = 0;
                matches.clear();
                prev = Some(self1);
            }

            let self_diff = *self2 - *self1;
            for (other1, other2) in other.beacons.unordered_combinations() {
                let other_diff = *other2 - *other1;
                if other_diff == self_diff || other_diff == -self_diff {
                    match_count += 1;
                    // Since we don't know if other1 or other2 is the one matching self1, store both
                    // along with how often we've seen them.
                    for e in [*other1, *other2] {
                        matches
                            .entry(e)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }

            // We need 12 matching beacons, that's 11 beacons + self1. The beacon from other that
            // matches self1 is the one with the highest count in matches. It should be in there
            // match_count times, with all other beacons appearing just once.
            if match_count >= 11 {
                let (matching_beacon, count) = matches.iter().max_by_key(|x| x.1).unwrap();
                assert_eq!(match_count, *count);
                // The offset between that beacon and self1 gives us the relative position of the
                // other scanner compared to self.
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
        assert_eq!(79, part1(AocInput::from_sample()));
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
