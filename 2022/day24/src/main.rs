// https://adventofcode.com/2022/day/24
use bitvec::prelude::*;
use std::{collections::HashSet, fmt::Display, path::Path};

use aoc::{
    aoc_input,
    dijkstra::{shortest_paths, Graph},
    grid::{Grid, GridBuilder, Point, PointDiff},
};

fn main() {
    let path = aoc_input();
    let part1 = part1(&path);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2(&path, part1));
}

// Find the quickest path through the valley with moving blizzards.
fn part1(path: impl AsRef<Path>) -> usize {
    let valley = Valley::from_file(path);
    let graph = ValleyGraph::new(valley.clone());
    let source = ValleyVertex(0, Point::new(0, 1));
    let dest = Point::new(valley.0.height() - 1, valley.0.width() - 2);
    graph.find_path(source, dest)
}

// Then head back to the start, and back to the end again.
fn part2(path: impl AsRef<Path>, there: usize) -> usize {
    let valley = Valley::from_file(path);
    let graph = ValleyGraph::new(valley.clone());
    let exit = Point::new(valley.0.height() - 1, valley.0.width() - 2);
    let source = ValleyVertex(there % graph.0.len(), exit);
    let dest = Point::new(0, 1);
    let back = graph.find_path(source, dest);
    println!("Back: {back}");
    let source = ValleyVertex((there + back) % graph.0.len(), Point::new(0, 1));
    let dest = exit;
    let again = graph.find_path(source, dest);
    println!("Again: {again}");

    there + back + again
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    // Bitfield of which directions are present.
    Floor(BitArray<[u8; 1]>),
}

impl Tile {
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

#[derive(Clone, PartialEq, Eq)]
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

    fn get_empty_tiles(&self) -> HashSet<Point> {
        self.0
            .cells()
            .filter_map(|(pos, tile)| tile.is_clear().then_some(pos))
            .collect()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct ValleyVertex(usize, Point);

struct ValleyGraph(Vec<HashSet<Point>>);

impl ValleyGraph {
    fn new(mut valley: Valley) -> Self {
        let mut empty_valley = valley.clone();
        for (_, tile) in empty_valley.0.cells_mut() {
            if let Tile::Floor(blizzards) = tile {
                *blizzards = Default::default();
            }
        }

        // Precompute all the valley blizzard positions (there aren't that many), and every blank
        // tile (non-wall with no blizzards) for that state. These are the vertices for the graph.
        let first_valley = valley.clone();
        let mut valleys = vec![valley.get_empty_tiles()];
        loop {
            valley = valley.move_blizzards(&empty_valley);
            if valley == first_valley {
                break;
            }

            valleys.push(valley.get_empty_tiles());
        }

        Self(valleys)
    }

    fn find_path(&self, source: ValleyVertex, dest: Point) -> usize {
        let paths = shortest_paths(self, &source);
        // Find the valley state where the exit tile has the shortest distance.
        paths
            .iter()
            .filter_map(|(vertex, info)| (vertex.1 == dest).then_some(info.distance))
            .min()
            .unwrap()
    }
}

impl Graph<ValleyVertex> for ValleyGraph {
    fn vertices(&self) -> HashSet<ValleyVertex> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(index, valley)| valley.iter().map(move |pos| ValleyVertex(index, *pos)))
            .collect()
    }

    fn neighbors(&self, v: &ValleyVertex) -> Vec<(ValleyVertex, usize)> {
        // Neighbors are points in the next valley state that are adjacent or equal to the current point.
        let index = (v.0 + 1) % self.0.len();
        let empty_tiles = &self.0[index];
        v.1.straight_neighbors()
            .chain([v.1])
            .filter_map(|nb| {
                empty_tiles
                    .contains(&nb)
                    .then_some((ValleyVertex(index, nb), 1))
            })
            .collect()
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
        let part1 = part1(aoc_sample_input());
        assert_eq!(54, part2(aoc_sample_input(), part1));
    }
}
