// https://adventofcode.com/2023/day/21

use aoc::{
    grid::{Grid, GridBuilder},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input(), 64));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// How many garden plots could be reached from the starting point in the given number of steps.
fn part1(input: AocInput, steps: usize) -> usize {
    let mut map = Map::from_input(input);
    for _ in 0..steps {
        map.walk();
    }

    map.count()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[derive(PartialEq, Eq)]
enum Tile {
    GardenPlot(bool, bool),
    Rock,
}

struct Map(Grid<Tile>);

impl Map {
    fn from_input(input: AocInput) -> Self {
        let grid = GridBuilder::from_input(input)
            .map(|_, cell| match cell {
                b'S' => Tile::GardenPlot(true, false),
                b'.' => Tile::GardenPlot(false, false),
                b'#' => Tile::Rock,
                _ => unreachable!(),
            })
            .build();

        Self(grid)
    }

    fn walk(&mut self) {
        for pos in self.0.bounding_rect().points() {
            if matches!(self.0[pos], Tile::GardenPlot(true, _)) {
                for nb in self.0.straight_neighbors(pos) {
                    if let Tile::GardenPlot(_, value) = &mut self.0[nb] {
                        *value = true
                    }
                }
            }
        }

        for (_, tile) in self.0.cells_mut() {
            if let Tile::GardenPlot(first, second) = tile {
                *first = *second;
                *second = false;
            }
        }
    }

    fn count(&self) -> usize {
        self.0
            .cells()
            .filter(|(_, tile)| matches!(tile, Tile::GardenPlot(true, _)))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(16, part1(AocInput::from_sample(), 6));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
