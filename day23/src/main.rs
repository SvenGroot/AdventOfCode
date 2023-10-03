// https://adventofcode.com/2022/day/23
use std::{collections::HashMap, fmt::Display, path::Path};

use aoc::{
    aoc_input, get_input,
    grid::{Grid, GridBuilder, Point, PointDiff, Rectangle, SubGrid},
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let mut grove = Grove(
        GridBuilder::from_file(&path)
            .map(|ch| match ch {
                b'.' => Tile::Empty,
                b'#' => Tile::Elf,
                _ => unreachable!(),
            })
            .extend(100, 100, b'.')
            .build(),
    );

    grove.move_elves(10);
    let bounded = grove.get_bounded();
    println!("{bounded}");
    bounded
        .values()
        .filter(|tile| **tile == Tile::Empty)
        .count()
}

fn part2(path: impl AsRef<Path>) -> usize {
    get_input(path).map(|_| 0).sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Elf,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match *self {
            Tile::Empty => '.',
            Tile::Elf => '#',
        };

        write!(f, "{}", ch)
    }
}

struct Grove(Grid<Tile>);

impl Grove {
    fn move_elves(&mut self, steps: usize) {
        for step in 0..steps {
            let mut proposed_moves: HashMap<Point, (Point, bool)> = HashMap::new();
            for (elf, _) in self.0.cells().filter(|(_, tile)| **tile == Tile::Elf) {
                if let Some(new_pos) = self.find_move(elf, step) {
                    // If the move is in the hash table, set it to true to indicate it was rejected.
                    // Otherwise, insert as false.
                    proposed_moves
                        .entry(new_pos)
                        .and_modify(|val| val.1 = true)
                        .or_insert((elf, false));
                }
            }

            for (new_pos, (old_pos, rejected)) in proposed_moves {
                if !rejected {
                    self.0[old_pos] = Tile::Empty;
                    self.0[new_pos] = Tile::Elf;
                }
            }
        }
    }

    fn get_bounded(&self) -> SubGrid<Tile> {
        let mut min_row = usize::MAX;
        let mut max_row = usize::MIN;
        let mut min_col = usize::MAX;
        let mut max_col = usize::MIN;
        for (pos, tile) in self.0.cells() {
            if *tile == Tile::Elf {
                min_row = min_row.min(pos.row());
                max_row = max_row.max(pos.row());
                min_col = min_col.min(pos.col());
                max_col = max_col.max(pos.col());
            }
        }

        let bounds = Rectangle::new(Point::new(min_row, min_col), Point::new(max_row, max_col));
        SubGrid::new(&self.0, bounds)
    }

    fn find_move(&self, elf: Point, step: usize) -> Option<Point> {
        if self
            .0
            .all_neighbors(elf)
            .all(|nb| self.0[nb] == Tile::Empty)
        {
            return None;
        }

        for index in 0..MOVES.len() {
            let index = (index + step) % MOVES.len();
            let mv = &MOVES[index];
            if self
                .0
                .neighbors(elf, &mv.look)
                .all(|nb| self.0[nb] == Tile::Empty)
            {
                return Some(elf + mv.dir);
            }
        }

        None
    }
}

struct Move {
    look: [PointDiff; 3],
    dir: PointDiff,
}

const MOVES: [Move; 4] = [
    Move {
        look: [PointDiff::UP, PointDiff::UP_LEFT, PointDiff::UP_RIGHT],
        dir: PointDiff::UP,
    },
    Move {
        look: [PointDiff::DOWN, PointDiff::DOWN_LEFT, PointDiff::DOWN_RIGHT],
        dir: PointDiff::DOWN,
    },
    Move {
        look: [PointDiff::LEFT, PointDiff::UP_LEFT, PointDiff::DOWN_LEFT],
        dir: PointDiff::LEFT,
    },
    Move {
        look: [PointDiff::RIGHT, PointDiff::UP_RIGHT, PointDiff::DOWN_RIGHT],
        dir: PointDiff::RIGHT,
    },
];

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(110, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(aoc_sample_input()));
    }
}
