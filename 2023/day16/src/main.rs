// https://adventofcode.com/2023/day/16

use std::{
    collections::HashSet,
    fmt::{Debug, Formatter},
};

use aoc::{
    grid::{Grid, GridBuilder, Point, PointDiff, Rotation},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Count the number of tiles energized by the beam.
fn part1(input: AocInput) -> usize {
    let mut contraption = Contraption::from_input(input);
    contraption.trace_beam(Point::default(), PointDiff::RIGHT);
    contraption.count_energized()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct Contraption(Grid<Tile>);

impl Contraption {
    fn from_input(input: AocInput) -> Self {
        let grid = GridBuilder::from_input(input)
            .map(|_, b| Tile {
                kind: match b {
                    b'.' => TileKind::Empty,
                    b'\\' => TileKind::MirrorLeft,
                    b'/' => TileKind::MirrorRight,
                    b'|' => TileKind::SplitterVertical,
                    b'-' => TileKind::SplitterHorizontal,
                    _ => unreachable!(),
                },
                energized: HashSet::new(),
            })
            .build();

        Self(grid)
    }

    fn trace_beam(&mut self, mut pos: Point, mut dir: PointDiff) {
        loop {
            if !self.0[pos].energized.insert(dir) {
                // We've been here before going the same direction; may be a loop.
                break;
            }

            // println!("{self:?}");
            let (dir1, dir2) = match self.0[pos].kind {
                TileKind::Empty => (dir, None),
                TileKind::MirrorLeft => {
                    if dir.is_horizontal() {
                        (dir.rotate(Rotation::Right), None)
                    } else {
                        (dir.rotate(Rotation::Left), None)
                    }
                }
                TileKind::MirrorRight => {
                    if dir.is_horizontal() {
                        (dir.rotate(Rotation::Left), None)
                    } else {
                        (dir.rotate(Rotation::Right), None)
                    }
                }
                TileKind::SplitterHorizontal => {
                    if dir.is_horizontal() {
                        (dir, None)
                    } else {
                        (PointDiff::LEFT, Some(PointDiff::RIGHT))
                    }
                }
                TileKind::SplitterVertical => {
                    if dir.is_horizontal() {
                        (PointDiff::UP, Some(PointDiff::DOWN))
                    } else {
                        (dir, None)
                    }
                }
            };

            if let Some(dir2) = dir2 {
                if let Some(next) = self.0.add_point(pos, dir2) {
                    self.trace_beam(next, dir2);
                }
            }

            if let Some(next) = self.0.add_point(pos, dir1) {
                pos = next;
                dir = dir1;
            } else {
                break;
            }
        }
    }

    fn count_energized(&self) -> usize {
        self.0
            .cells()
            .filter(|(_, tile)| !tile.energized.is_empty())
            .count()
    }
}

impl Debug for Contraption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0
            .write_mapped(f, |tile| if !tile.energized.is_empty() { '#' } else { '.' })
    }
}

struct Tile {
    kind: TileKind,
    energized: HashSet<PointDiff>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Empty,
    MirrorLeft,
    MirrorRight,
    SplitterVertical,
    SplitterHorizontal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(46, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
