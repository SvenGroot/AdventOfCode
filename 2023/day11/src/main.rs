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
    println!("Part 2: {}", part2(AocInput::from_input(), 999999));
}

// Double empty rows and columns, then get shortest path between each pair of galaxies.
fn part1(input: AocInput) -> usize {
    let universe = Universe::from_input(input, 1);
    universe.shortest_paths_sum()
}

// Same, but expand empty rows and columns 1,000,000 times.
// N.B. This is a parameter because the test uses 100 times.
fn part2(input: AocInput, expansion_amount: usize) -> usize {
    let universe = Universe::from_input(input, expansion_amount);
    universe.shortest_paths_sum()
}

// Just casually modelling the entire universe.
#[derive(Debug)]
struct Universe {
    space: Grid<Space>,
    expanded_rows: HashSet<usize>,
    expanded_cols: HashSet<usize>,
    expansion_amount: usize,
}

impl Universe {
    fn from_input(input: AocInput, expansion_amount: usize) -> Self {
        let space = GridBuilder::from_input(input)
            .map(|_, value| match value {
                b'#' => Space::Galaxy,
                b'.' => Space::Empty,
                _ => unreachable!(),
            })
            .build();

        let expanded_rows = space
            .rows()
            .enumerate()
            .filter_map(|(row_index, row)| {
                row.iter()
                    .all(|value| *value == Space::Empty)
                    .then_some(row_index)
            })
            .collect();

        let expanded_cols = space
            .cols()
            .enumerate()
            .filter_map(|(col_index, mut col)| {
                col.all(|value| *value == Space::Empty).then_some(col_index)
            })
            .collect();

        Self {
            space,
            expanded_rows,
            expanded_cols,
            expansion_amount,
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
        self.path_length(galaxy1.row(), galaxy2.row(), &self.expanded_rows)
            + self.path_length(galaxy1.col(), galaxy2.col(), &self.expanded_cols)
    }

    fn path_length(&self, start: usize, end: usize, expanded: &HashSet<usize>) -> usize {
        let min = start.min(end);
        let max = start.max(end);
        let expanded_count = (min..max).filter(|value| expanded.contains(value)).count();

        (max - min) + (expanded_count * self.expansion_amount)
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
        assert_eq!(8410, part2(AocInput::from_sample(), 99));
    }
}
