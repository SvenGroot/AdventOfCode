// https://adventofcode.com/2023/day/11

use std::collections::HashSet;

use aoc::{
    grid::{Grid, GridBuilder, Point},
    input::AocInput,
    iterator::IteratorExt,
    slice::SliceExt,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Double empty rows and columns, then get shortest path between each pair of galaxies.
fn part1(input: AocInput) -> usize {
    let universe = Universe::from_input(input);
    universe.shortest_paths_sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

// Just casually modelling the entire universe.
#[derive(Debug)]
struct Universe {
    space: Grid<Space>,
    double_rows: HashSet<usize>,
    double_cols: HashSet<usize>,
}

impl Universe {
    fn from_input(input: AocInput) -> Self {
        let space = GridBuilder::from_input(input)
            .map(|_, value| match value {
                b'#' => Space::Galaxy,
                b'.' => Space::Empty,
                _ => unreachable!(),
            })
            .build();

        let double_rows = space
            .rows()
            .enumerate()
            .filter_map(|(row_index, row)| {
                row.iter()
                    .all(|value| *value == Space::Empty)
                    .then_some(row_index)
            })
            .collect();

        let double_cols = space
            .cols()
            .enumerate()
            .filter_map(|(col_index, mut col)| {
                col.all(|value| *value == Space::Empty).then_some(col_index)
            })
            .collect();

        Self {
            space,
            double_rows,
            double_cols,
        }
    }

    fn shortest_paths_sum(&self) -> usize {
        let galaxies = self
            .space
            .cells()
            .filter_map(|(pos, value)| (*value == Space::Galaxy).then_some(pos))
            .into_vec();

        galaxies
            .combinations()
            .map(|(galaxy1, galaxy2)| self.shortest_path(*galaxy1, *galaxy2))
            .sum()
    }

    fn shortest_path(&self, galaxy1: Point, galaxy2: Point) -> usize {
        // Shortest path is just manhattan distance, but we have to account for expanded rows and
        // columns.
        Self::path_length(galaxy1.row(), galaxy2.row(), &self.double_rows)
            + Self::path_length(galaxy1.col(), galaxy2.col(), &self.double_cols)
    }

    fn path_length(start: usize, end: usize, doubles: &HashSet<usize>) -> usize {
        let min = start.min(end);
        let max = start.max(end);
        let double_count = (min..max).filter(|value| doubles.contains(value)).count();

        (max - min) + double_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Galaxy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(374, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
