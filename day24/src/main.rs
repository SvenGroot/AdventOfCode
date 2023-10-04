// https://adventofcode.com/2022/day/24
use bitvec::prelude::*;
use std::{fmt::Display, path::Path};

use aoc::{
    aoc_input, get_input,
    grid::{Grid, GridBuilder, Point, PointDiff},
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let mut valley = Valley::from_file(path);
    println!("{}", valley.0);
    for _ in 0..5 {
        valley.move_blizzards();
        println!("{}", valley.0);
    }
    0
}

fn part2(path: impl AsRef<Path>) -> usize {
    get_input(path).map(|_| 0).sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    // Bitfield of which directions are present.
    Floor(BitArray<[u8; 1]>),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match *self {
            Tile::Wall => '#',
            Tile::Floor(blizzards) => {
                let count = blizzards.count_ones();
                if count == 0 {
                    '.'
                } else if count > 1 {
                    char::from_digit(count as u32, 10).unwrap()
                } else {
                    PointDiff::STRAIGHT_NEIGHBORS[blizzards.first_one().unwrap()]
                        .get_dir_char()
                        .unwrap()
                }
            }
        };

        write!(f, "{ch}")
    }
}

struct Valley(Grid<Tile>);

impl Valley {
    fn from_file(path: impl AsRef<Path>) -> Self {
        Self(
            GridBuilder::from_file(path)
                .map(|ch| match ch {
                    b'#' => Tile::Wall,
                    b'.' => Tile::Floor(Default::default()),
                    ch => {
                        let dir = PointDiff::from_char(ch, b'^', b'>', b'v', b'<').unwrap();
                        let index = PointDiff::STRAIGHT_NEIGHBORS
                            .iter()
                            .position(|nb| *nb == dir)
                            .unwrap();

                        let mut array = BitArray::default();
                        array.set(index, true);
                        Tile::Floor(array)
                    }
                })
                .build(),
        )
    }

    fn move_blizzards(&mut self) {
        for pos in self.0.bounding_rect().points() {
            if let Tile::Floor(blizzards) = self.0[pos] {
                for i in 0..4 {
                    if blizzards[i] {
                        let dir = PointDiff::STRAIGHT_NEIGHBORS[i];
                        let mut new_pos = pos + dir;
                        if self.0[new_pos] == Tile::Wall {
                            new_pos = self.wrap_blizzard(pos, dir);
                        }

                        let Tile::Floor(new_blizzards) = &mut self.0[new_pos] else {
                            unreachable!();
                        };

                        // Use upper 4 bits to store new positions.
                        new_blizzards.set(i + 4, true);
                    }
                }
            }
        }

        for (_, tile) in self.0.cells_mut() {
            if let Tile::Floor(blizzards) = tile {
                let values = blizzards.as_raw_mut_slice();

                // Move pending blizzards into active position, and clear pending.
                values[0] >>= 4;
            }
        }
    }

    fn wrap_blizzard(&self, pos: Point, dir: PointDiff) -> Point {
        match dir {
            PointDiff::UP => Point::new(self.0.height() - 2, pos.col()),
            PointDiff::DOWN => Point::new(1, pos.col()),
            PointDiff::LEFT => Point::new(pos.row(), self.0.width() - 2),
            PointDiff::RIGHT => Point::new(pos.row(), 1),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(18, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(aoc_sample_input()));
    }
}
