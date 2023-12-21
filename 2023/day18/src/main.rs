// https://adventofcode.com/2023/day/18

use std::{fmt::Display, str::FromStr};

use aoc::{
    grid::{Grid, Point, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Calculate the trench size.
// N.B. Slow on debug mode.
fn part1(input: AocInput) -> usize {
    let map = DigMap::from_input(input);
    map.trench_size()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct DigMap(Grid<Tile>);

impl DigMap {
    fn from_input(input: AocInput) -> Self {
        const SIZE: usize = 1000;
        let mut grid = Grid::new(SIZE, SIZE, Tile::Unknown);
        let mut current = Point::new(SIZE / 2, SIZE / 2);
        grid[current] = Tile::Inside;
        println!("Following plan.");
        for instruction in input.parsed::<DigInstruction>() {
            for _ in 0..instruction.length {
                current += instruction.dir;
                grid[current] = Tile::Inside;
            }
        }

        for (_, tile) in grid.edge_cells_mut() {
            if *tile == Tile::Unknown {
                *tile = Tile::Outside
            }
        }

        println!("Marking outside");
        grid.grow_value(|t| *t == Tile::Outside, |t| *t == Tile::Unknown);
        println!("Marking inside");
        for (_, tile) in grid.cells_mut() {
            if *tile == Tile::Unknown {
                *tile = Tile::Inside
            }
        }

        Self(grid)
    }

    fn trench_size(&self) -> usize {
        self.0
            .cells()
            .filter(|(_, cell)| **cell == Tile::Inside)
            .count()
    }
}

impl Display for DigMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .write_mapped(f, |cell| if *cell == Tile::Inside { '#' } else { '.' })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Inside,
    Outside,
}

struct DigInstruction {
    dir: PointDiff,
    length: usize,
    _color: String,
}

impl FromStr for DigInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let dir = parts.next().unwrap();
        let dir = PointDiff::from_char(dir.as_bytes()[0], [b'U', b'R', b'D', b'L']).unwrap();
        let length = parts.next().unwrap().parse().unwrap();
        let color = parts.next().unwrap().into();
        Ok(Self {
            dir,
            length,
            _color: color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(62, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
