// https://adventofcode.com/2023/day/13

use aoc::{
    grid::{Grid, GridBuilder, Point, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find the horizontal or vertical line where each pattern is reflected.
fn part1(input: AocInput) -> usize {
    input
        .into_vec()
        .split(|line| line.is_empty())
        .map(|grid| Pattern::from(grid).find_reflection())
        .sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct Pattern(Grid<Tile>);

impl Pattern {
    fn find_reflection(&self) -> usize {
        for col in 0..self.0.width() - 1 {
            if (0..self.0.height()).all(|row| self.check_reflection(Point::new(row, col), false)) {
                return col + 1;
            }
        }

        for row in 0..self.0.height() - 1 {
            if (0..self.0.width()).all(|col| self.check_reflection(Point::new(row, col), true)) {
                return (row + 1) * 100;
            }
        }

        unreachable!();
    }

    fn check_reflection(&self, pos: Point, vertical: bool) -> bool {
        let (dir1, dir2) = if vertical {
            (PointDiff::UP, PointDiff::DOWN)
        } else {
            (PointDiff::LEFT, PointDiff::RIGHT)
        };

        let mut side1 = pos;
        let mut side2 = pos + dir2;
        loop {
            if self.0[side1] != self.0[side2] {
                return false;
            }

            side1 = if let Some(next) = self.0.add_point(side1, dir1) {
                next
            } else {
                break;
            };

            side2 = if let Some(next) = self.0.add_point(side2, dir2) {
                next
            } else {
                break;
            };
        }

        true
    }
}

impl From<&[String]> for Pattern {
    fn from(value: &[String]) -> Self {
        let grid = GridBuilder::from_lines(value.iter())
            .map(|_, b| match b {
                b'.' => Tile::Ash,
                b'#' => Tile::Rock,
                _ => unreachable!(),
            })
            .build();

        Self(grid)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(405, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
