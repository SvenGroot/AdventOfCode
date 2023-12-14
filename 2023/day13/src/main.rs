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
        .map(|grid| Pattern::from(grid).find_reflection(None).unwrap())
        .sum()
}

// Find one spot in the pattern to change that gets a different reflection line, and use those
// instead.
fn part2(input: AocInput) -> usize {
    input
        .into_vec()
        .split(|line| line.is_empty())
        .map(|grid| Pattern::from(grid).find_smudge())
        .sum()
}

struct Pattern(Grid<bool>);

impl Pattern {
    fn find_smudge(&mut self) -> usize {
        let original = self.find_reflection(None).unwrap();
        for pos in self.0.bounding_rect().points() {
            self.0[pos] = !self.0[pos];
            if let Some(result) = self.find_reflection(Some(original)) {
                return result;
            }

            self.0[pos] = !self.0[pos];
        }

        unreachable!();
    }

    fn find_reflection(&self, exclude: Option<usize>) -> Option<usize> {
        for col in 0..self.0.width() - 1 {
            if (0..self.0.height()).all(|row| self.check_reflection(Point::new(row, col), false)) {
                let result = Some(col + 1);
                if result != exclude {
                    return result;
                }
            }
        }

        for row in 0..self.0.height() - 1 {
            if (0..self.0.width()).all(|col| self.check_reflection(Point::new(row, col), true)) {
                let result = Some((row + 1) * 100);
                if result != exclude {
                    return result;
                }
            }
        }

        None
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
                b'.' => true,
                b'#' => false,
                _ => unreachable!(),
            })
            .build();

        Self(grid)
    }
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
        assert_eq!(400, part2(AocInput::from_sample()));
    }
}
