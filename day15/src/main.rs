#[macro_use]
extern crate text_io;

use std::{collections::HashSet, path::Path, str::FromStr};

use aoc::{aoc_input, get_input, grid::PointDiff};

fn main() {
    println!("Start");
    let path = aoc_input();
    println!("Part 1: {}", part1(&path, 2_000_000));
    println!("Part 2: {}", part2(&path, 4_000_000));
}

fn part1(path: impl AsRef<Path>, row: isize) -> usize {
    let map = CoverageMap::from_file(path);
    map.coverage_on_row(row)
}

fn part2(path: impl AsRef<Path>, max: isize) -> isize {
    let map = CoverageMap::from_file(path);
    map.uncovered_between(PointDiff::default(), PointDiff::new(max, max))
        .unwrap()
}

struct CoverageMap(Vec<Sensor>);

impl CoverageMap {
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let sensors = get_input(path).map(|line| line.parse().unwrap()).collect();
        Self(sensors)
    }

    pub fn coverage_on_row(&self, row: isize) -> usize {
        let coverage = self.sensor_coverage_on_row(row);
        let beacons: HashSet<_> = self
            .0
            .iter()
            .filter_map(|sensor| (sensor.beacon.row() == row).then_some(sensor.beacon))
            .collect();

        let mut current = isize::MIN;
        let mut count = 0;
        for (start, end) in coverage {
            let start = start.max(current);
            if start <= end {
                let beacon_count = beacons
                    .iter()
                    .filter(|b| b.col() >= start && b.col() <= end)
                    .count();

                count += end - start + 1 - beacon_count as isize;
                current = end + 1;
            }
        }

        count as usize
    }

    pub fn uncovered_between(&self, start: PointDiff, end: PointDiff) -> Option<isize> {
        for row in start.row()..=end.row() {
            if let Some(col) = self.uncovered_on_row(row, start.col(), end.col()) {
                return Some(col * 4_000_000 + row);
            }
        }

        None
    }

    fn uncovered_on_row(&self, row: isize, start_col: isize, end_col: isize) -> Option<isize> {
        let coverage = self.sensor_coverage_on_row(row);
        let mut current = isize::MIN;
        for (start, end) in coverage {
            let start = start.max(current);
            if start <= end {
                if current >= start_col && current < end_col && start > current {
                    return Some(current);
                }

                current = end + 1;
            }
        }

        None
    }

    fn sensor_coverage_on_row(&self, row: isize) -> Vec<(isize, isize)> {
        let mut coverage: Vec<_> = self
            .0
            .iter()
            .filter_map(|s| s.coverage_at_row(row))
            .collect();

        coverage.sort_by_key(|(start, _)| *start);
        coverage
    }
}

struct Sensor {
    location: PointDiff,
    beacon: PointDiff,
}

impl Sensor {
    pub fn coverage_at_row(&self, row: isize) -> Option<(isize, isize)> {
        let distance = self.coverage_distance();
        let vert_dist = (row - self.location.row()).abs();
        let hor_dist = distance - vert_dist;
        if hor_dist < 0 {
            return None;
        }

        let start = self.location.col() - hor_dist;
        let end = self.location.col() + hor_dist;
        Some((start, end))
    }

    pub fn coverage_distance(&self) -> isize {
        let relative = (self.beacon - self.location).abs();
        relative.row() + relative.col()
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sensor_row: isize;
        let sensor_col: isize;
        let beacon_row: isize;
        let beacon_col: isize;
        scan!(s.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            sensor_col, sensor_row, beacon_col, beacon_row);

        Ok(Self {
            location: PointDiff::new(sensor_row, sensor_col),
            beacon: PointDiff::new(beacon_row, beacon_col),
        })
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(26, part1(aoc_sample_input(), 10));
    }

    #[test]
    fn test_part2() {
        assert_eq!(56000011, part2(aoc_sample_input(), 20));
    }
}
