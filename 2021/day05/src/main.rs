// https://adventofcode.com/2021/day/5

use std::str::FromStr;

use aoc::{
    grid::{Grid, Point},
    input::AocInput,
    iterator::IteratorExt,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Number of points covered by more than one line, considering only straight lines.
fn part1(input: AocInput) -> usize {
    let map = VentMap::new(
        input
            .parsed::<LineSegment>()
            .filter(|line| line.start.row() == line.end.row() || line.start.col() == line.end.col())
            .into_vec(),
    );
    map.overlap_count()
}

// Number of points covered by more than one line, considering all lines.
fn part2(input: AocInput) -> usize {
    let map = VentMap::new(input.parsed::<LineSegment>().into_vec());
    map.overlap_count()
}

struct VentMap(Grid<usize>);

impl VentMap {
    fn new(input: Vec<LineSegment>) -> Self {
        // Get a point with the max row and column of all points.
        let max = input.iter().fold(Point::default(), |max, line| {
            Point::new(
                max.row().max(line.start.row()).max(line.end.row()),
                max.col().max(line.start.col()).max(line.end.col()),
            )
        });

        let mut grid = Grid::new(max.row() + 1, max.col() + 1, 0);

        for line in &input {
            let line = line.start.line_to(line.end).unwrap();
            for pt in line {
                grid[pt] += 1;
            }
        }

        VentMap(grid)
    }

    fn overlap_count(&self) -> usize {
        self.0.cells().filter(|(_, cell)| **cell > 1).count()
    }
}

struct LineSegment {
    start: Point,
    end: Point,
}

impl FromStr for LineSegment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        Ok(LineSegment {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2(AocInput::from_sample()));
    }
}
