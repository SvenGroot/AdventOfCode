// https://adventofcode.com/2023/day/10

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
    pipes.loop_length() / 2
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct PipeNetwork(Grid<Tile>);

impl PipeNetwork {
    fn from_input(input: AocInput) -> Self {
        let grid = GridBuilder::from_input(input)
            .map(|pos, cell| match cell {
                b'|' => Tile::new_pipe(pos, PointDiff::UP, PointDiff::DOWN),
                b'-' => Tile::new_pipe(pos, PointDiff::LEFT, PointDiff::RIGHT),
                b'L' => Tile::new_pipe(pos, PointDiff::UP, PointDiff::RIGHT),
                b'J' => Tile::new_pipe(pos, PointDiff::UP, PointDiff::LEFT),
                b'7' => Tile::new_pipe(pos, PointDiff::DOWN, PointDiff::LEFT),
                b'F' => Tile::new_pipe(pos, PointDiff::DOWN, PointDiff::RIGHT),
                b'S' => Tile::Start,
                _ => Tile::Ground,
            })
            .build();

        Self(grid)
    }

    fn loop_length(&self) -> usize {
        let (start, _) = self
            .0
            .cells()
            .find(|(_, tile)| **tile == Tile::Start)
            .unwrap();

        let mut current = start
            .straight_neighbors()
            .find(|&p| {
                if let Tile::Pipe(pipe) = &self.0[p] {
                    pipe.0 == start || pipe.1 == start
                } else {
                    false
                }
            })
            .unwrap();

        let mut next = self.0[current].connection(start).unwrap();
        let mut steps = 0;
        while self.0[next] != Tile::Start {
            let new_next = self.0[next].connection(current).unwrap();
            current = next;
            next = new_next;
            steps += 1;
        }

        // Add two for the step from the start and back to the start.
        steps + 2
    }
}

#[derive(PartialEq, Eq)]
enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
}

impl Tile {
    fn new_pipe(pos: Point, c1: PointDiff, c2: PointDiff) -> Self {
        if let Some(pipe) = Pipe::new(pos, c1, c2) {
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

#[derive(PartialEq, Eq)]
struct Pipe(Point, Point);

impl Pipe {
    fn new(pos: Point, c1: PointDiff, c2: PointDiff) -> Option<Self> {
        Some(Self(pos.add_diff(c1)?, pos.add_diff(c2)?))
    }

    fn connection(&self, pos: Point) -> Option<Point> {
        if self.0 == pos {
            Some(self.1)
        } else if self.1 == pos {
            Some(self.0)
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
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
