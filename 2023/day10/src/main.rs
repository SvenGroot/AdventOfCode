// https://adventofcode.com/2023/day/10

use std::fmt::Display;

use aoc::{
    grid::{Grid, GridBuilder, Point, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Determine the point furthest away from the start along the loop.
fn part1(input: AocInput) -> usize {
    let pipes = PipeNetwork::from_input(input);
    let mut steps = 0;
    pipes.follow_loop(|_| steps += 1);
    steps / 2
}

// Count the number of tiles enclosed by the loop.
fn part2(input: AocInput) -> usize {
    println!("Running");
    let mut pipes = PipeNetwork::from_input(input);
    pipes.isolate_loop()
}

struct PipeNetwork(Grid<Tile>);

impl PipeNetwork {
    fn from_input(input: AocInput) -> Self {
        let grid = GridBuilder::from_input(input)
            .map(|pos, cell| match cell {
                b'|' => Tile::new_pipe(pos, PointDiff::UP, PointDiff::DOWN, cell),
                b'-' => Tile::new_pipe(pos, PointDiff::LEFT, PointDiff::RIGHT, cell),
                b'L' => Tile::new_pipe(pos, PointDiff::UP, PointDiff::RIGHT, cell),
                b'J' => Tile::new_pipe(pos, PointDiff::UP, PointDiff::LEFT, cell),
                b'7' => Tile::new_pipe(pos, PointDiff::DOWN, PointDiff::LEFT, cell),
                b'F' => Tile::new_pipe(pos, PointDiff::DOWN, PointDiff::RIGHT, cell),
                b'S' => Tile::Start,
                _ => Tile::Ground,
            })
            .build();

        Self(grid)
    }

    fn follow_loop(&self, mut f: impl FnMut(Point)) {
        let (start, _) = self
            .0
            .cells()
            .find(|(_, tile)| **tile == Tile::Start)
            .unwrap();

        f(start);
        let mut current = start
            .straight_neighbors()
            .find(|&p| {
                if let Tile::Pipe(pipe) = &self.0[p] {
                    pipe.c1 == start || pipe.c2 == start
                } else {
                    false
                }
            })
            .unwrap();

        f(current);
        let mut next = self.0[current].connection(start).unwrap();
        while self.0[next] != Tile::Start {
            let new_next = self.0[next].connection(current).unwrap();
            current = next;
            next = new_next;
            f(current)
        }
    }

    fn get_loop(&self) -> Vec<Point> {
        let mut result = Vec::new();
        self.follow_loop(|pos| result.push(pos));
        result
    }

    fn isolate_loop(&mut self) -> usize {
        let loop_parts = self.get_loop();

        // Mark all tiles that aren't part of the loop as ground.
        for (pos, tile) in self.0.cells_mut() {
            if *tile != Tile::Ground && !loop_parts.contains(&pos) {
                *tile = Tile::Ground;
            }
        }

        println!("Cleaned:");
        println!("{self}");

        // Now mark all tiles on either side of the loop consistently as either side1 or side2. At
        // this point we don't know which side is inside or outside, but doing this will properly
        // mark areas that can be reached by squeezing between pipes using whatever side will end up
        // being outside.
        self.mark_loop_sides(loop_parts);

        println!("Loop edges marked:");
        println!("{self}");

        // Now expand the sides until there is no more new work.
        let rect = self.0.bounding_rect();
        loop {
            let mut new_tiles = false;
            for pos in rect.points() {
                let tile = self.0[pos];
                if tile == Tile::Side1 || tile == Tile::Side2 {
                    for nb in self.0.straight_neighbors(pos) {
                        if self.0[nb] == Tile::Ground {
                            new_tiles = true;
                            self.0[nb] = tile;
                        }
                    }
                }
            }

            if !new_tiles {
                break;
            }
        }

        // Find a side touching the edge, which is the outside.
        let (_, outside) = self
            .0
            .edge_cells()
            .find(|(_, tile)| **tile == Tile::Side1 || **tile == Tile::Side2)
            .unwrap();

        let inside = if *outside == Tile::Side1 {
            Tile::Side2
        } else {
            Tile::Side1
        };

        self.count_tile(inside)
    }

    fn mark_loop_sides(&mut self, loop_parts: Vec<Point>) {
        for window in loop_parts.windows(2) {
            let Tile::Pipe(pipe) = self.0[window[1]] else {
                assert_eq!(self.0[window[1]], Tile::Start);
                continue;
            };

            if window[0].col() < window[1].col() {
                match pipe.kind {
                    b'-' => self.set_sides(window[1], PointDiff::UP, PointDiff::DOWN),
                    b'7' => {
                        self.set_same_side(window[1], PointDiff::UP, PointDiff::RIGHT, Tile::Side1)
                    }
                    b'J' => self.set_same_side(
                        window[1],
                        PointDiff::DOWN,
                        PointDiff::RIGHT,
                        Tile::Side2,
                    ),
                    _ => unreachable!(),
                }
            } else if window[0].col() > window[1].col() {
                match pipe.kind {
                    b'-' => self.set_sides(window[1], PointDiff::DOWN, PointDiff::UP),
                    b'L' => {
                        self.set_same_side(window[1], PointDiff::DOWN, PointDiff::LEFT, Tile::Side1)
                    }
                    b'F' => {
                        self.set_same_side(window[1], PointDiff::UP, PointDiff::LEFT, Tile::Side2)
                    }
                    _ => unreachable!(),
                }
            } else if window[0].row() < window[1].row() {
                match pipe.kind {
                    b'|' => self.set_sides(window[1], PointDiff::RIGHT, PointDiff::LEFT),
                    b'J' => self.set_same_side(
                        window[1],
                        PointDiff::DOWN,
                        PointDiff::RIGHT,
                        Tile::Side1,
                    ),
                    b'L' => {
                        self.set_same_side(window[1], PointDiff::DOWN, PointDiff::LEFT, Tile::Side2)
                    }
                    _ => unreachable!(),
                }
            } else {
                assert!(window[0].row() > window[1].row());
                match pipe.kind {
                    b'|' => self.set_sides(window[1], PointDiff::LEFT, PointDiff::RIGHT),
                    b'F' => {
                        self.set_same_side(window[1], PointDiff::UP, PointDiff::LEFT, Tile::Side1)
                    }
                    b'7' => {
                        self.set_same_side(window[1], PointDiff::UP, PointDiff::RIGHT, Tile::Side2)
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn count_tile(&self, value: Tile) -> usize {
        self.0.cells().filter(|(_, tile)| **tile == value).count()
    }

    fn set_side(&mut self, pos: Point, dir: PointDiff, value: Tile) {
        if let Some(side) = self.0.add_point(pos, dir) {
            if self.0[side] == Tile::Ground {
                self.0[side] = value;
            }
        }
    }

    fn set_sides(&mut self, pos: Point, side1: PointDiff, side2: PointDiff) {
        self.set_side(pos, side1, Tile::Side1);
        self.set_side(pos, side2, Tile::Side2);
    }

    fn set_same_side(&mut self, pos: Point, side1: PointDiff, side2: PointDiff, value: Tile) {
        self.set_side(pos, side1, value);
        self.set_side(pos, side2, value);
    }
}

impl Display for PipeNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.write_mapped(f, |tile| match tile {
            Tile::Ground => '.',
            Tile::Side1 => '1',
            Tile::Side2 => '2',
            Tile::Start => 'S',
            Tile::Pipe(pipe) => pipe.kind as char,
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
    Side1,
    Side2,
}

impl Tile {
    fn new_pipe(pos: Point, c1: PointDiff, c2: PointDiff, kind: u8) -> Self {
        if let Some(pipe) = Pipe::new(pos, c1, c2, kind) {
            Self::Pipe(pipe)
        } else {
            // A pipe that connects to a position off the grid can't be part of the loop, so just
            // treat it as ground.
            Self::Ground
        }
    }

    fn connection(&self, pos: Point) -> Option<Point> {
        let Tile::Pipe(pipe) = self else {
            return None;
        };

        pipe.connection(pos)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Pipe {
    c1: Point,
    c2: Point,
    kind: u8,
}

impl Pipe {
    fn new(pos: Point, c1: PointDiff, c2: PointDiff, kind: u8) -> Option<Self> {
        Some(Self {
            c1: pos.add_diff(c1)?,
            c2: pos.add_diff(c2)?,
            kind,
        })
    }

    fn connection(&self, pos: Point) -> Option<Point> {
        if self.c1 == pos {
            Some(self.c2)
        } else if self.c2 == pos {
            Some(self.c1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            10,
            part2(AocInput::from_file(AocInput::get_custom_path(
                "day10_part2",
                true
            )))
        );
    }
}
