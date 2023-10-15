// https://adventofcode.com/2021/day/13

use std::fmt::Display;

use aoc::{
    grid::{Grid, Point, Rectangle},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    part2(AocInput::from_input());
}

// Fold the grid once, and count visible dots.
fn part1(input: AocInput) -> usize {
    let mut grid = CodeGrid::from_input(input);
    grid.fold_once();
    grid.grid.cells().filter(|(_, cell)| **cell).count()
}

// Fold as instructed, display the result.
fn part2(input: AocInput) {
    let mut grid = CodeGrid::from_input(input);
    grid.fold_all();
    println!("Part 2:");
    println!("{grid}");
}

struct CodeGrid {
    grid: Grid<bool>,
    folds: Vec<Point>,
}

impl CodeGrid {
    fn from_input(mut input: AocInput) -> Self {
        let grid = Grid::from_string_points(
            input.by_ref().take_while(|line| !line.is_empty()),
            false,
            true,
        );
        let folds = input
            .map(|line| {
                let (along, value) = line.split_once('=').unwrap();
                if along == "fold along y" {
                    Point::new(value.parse().unwrap(), 0)
                } else {
                    Point::new(0, value.parse().unwrap())
                }
            })
            .collect();

        CodeGrid { grid, folds }
    }

    fn fold_once(&mut self) {
        Self::fold_helper(&mut self.grid, self.folds[0])
    }

    fn fold_all(&mut self) {
        for fold in &self.folds {
            Self::fold_helper(&mut self.grid, *fold);
        }
    }

    fn fold_helper(grid: &mut Grid<bool>, fold: Point) {
        let rect = grid.bounding_rect();
        let rect = Rectangle::new(rect.top_left() + fold, rect.bottom_right());
        for pos in rect.points() {
            let fold_pos = if fold.row() == 0 {
                Point::new(pos.row(), 2 * fold.col() - pos.col())
            } else {
                Point::new(2 * fold.row() - pos.row(), pos.col())
            };

            grid[fold_pos] |= grid[pos];
        }

        let (new_height, new_width) = if fold.row() == 0 {
            (grid.height(), fold.col())
        } else {
            (fold.row(), grid.width())
        };

        grid.shrink(new_height, new_width);
    }
}

impl Display for CodeGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.rows() {
            for item in row {
                let ch = if *item { '#' } else { '.' };
                write!(f, "{ch}")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(17, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        // No real test here, just run it to show the output for the sample.
        part2(AocInput::from_sample());
    }
}
