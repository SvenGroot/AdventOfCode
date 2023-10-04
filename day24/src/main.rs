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
    let valley = Valley::from_file(path);
    let mut empty_valley = valley.clone();
    for (_, tile) in empty_valley.0.cells_mut() {
        if let Tile::Floor(blizzards) = tile {
            *blizzards = Default::default();
        }
    }

    let mut best = 30;
    Simulation::new(valley).run(&empty_valley, &mut best)
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

impl Tile {
    // fn get_blizzards(&self) -> &BitArray<[u8; 1]> {
    //     match self {
    //         Tile::Floor(blizzards) => blizzards,
    //         _ => panic!("Not a floor."),
    //     }
    // }

    fn get_blizzards_mut(&mut self) -> &mut BitArray<[u8; 1]> {
        match self {
            Tile::Floor(blizzards) => blizzards,
            _ => panic!("Not a floor."),
        }
    }

    fn is_clear(&self) -> bool {
        match self {
            Tile::Floor(blizzards) => !blizzards.any(),
            Tile::Wall => false,
        }
    }
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

#[derive(Clone)]
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

    fn move_blizzards(&self, empty_valley: &Valley) -> Self {
        let mut target = empty_valley.clone();
        for pos in self.0.bounding_rect().points() {
            if let Tile::Floor(blizzards) = self.0[pos] {
                for i in 0..4 {
                    if blizzards[i] {
                        let dir = PointDiff::STRAIGHT_NEIGHBORS[i];
                        let mut new_pos = pos + dir;
                        if self.0[new_pos] == Tile::Wall {
                            new_pos = self.wrap_blizzard(pos, dir);
                        }

                        let new_blizzards = target.0[new_pos].get_blizzards_mut();

                        new_blizzards.set(i, true);
                    }
                }
            }
        }

        target
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

struct Simulation {
    valley: Valley,
    player: Point,
    wait_count: usize,
    minute: usize,
}

impl Simulation {
    fn new(valley: Valley) -> Self {
        Self {
            valley,
            player: Point::new(0, 1),
            wait_count: 0,
            minute: 0,
        }
    }

    fn run(self, empty_valley: &Valley, best: &mut usize) -> usize {
        if self.minute >= *best {
            return usize::MAX;
        }

        let new_valley = self.valley.move_blizzards(empty_valley);

        // Check if we can move to the tile adjacent to the exit tile.
        if (self.player == Point::new(self.valley.0.height() - 3, self.valley.0.width() - 2)
            || self.player == Point::new(self.valley.0.height() - 2, self.valley.0.width() - 3))
            && new_valley.0[Point::new(self.valley.0.height() - 2, self.valley.0.width() - 2)]
                .is_clear()
        {
            *best = self.minute;
            println!("New best {best}");
            return self.minute + 2;
        }

        // Check the possible moves.
        // N.B. Can't walk off the grid thanks to walls, so skip that check.
        let mut min_minutes = usize::MAX;
        for nb in self.player.straight_neighbors() {
            if nb != Point::new(0, 1) && new_valley.0[nb].is_clear() {
                let next = Simulation {
                    valley: new_valley.clone(),
                    player: nb,
                    wait_count: 0,
                    minute: self.minute + 1,
                };

                min_minutes = next.run(empty_valley, best).min(min_minutes);
            }
        }

        // Check if we can wait.
        let max_wait = self.valley.0.width().max(self.valley.0.height()) - 2;
        if new_valley.0[self.player].is_clear() && self.wait_count < max_wait {
            let next = Simulation {
                valley: new_valley,
                player: self.player,
                wait_count: self.wait_count + 1,
                minute: self.minute + 1,
            };

            min_minutes = next.run(empty_valley, best).min(min_minutes);
        }

        min_minutes
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
