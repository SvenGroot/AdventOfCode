// https://adventofcode.com/2023/day/14

use std::collections::HashMap;

use aoc::{
    grid::{Grid, GridBuilder, Point, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Calculate the load on the north columns after tilting the platform north.
fn part1(input: AocInput) -> usize {
    let mut platform = Platform::from_input(input);
    platform.tilt(PointDiff::UP);
    platform.calculate_load()
}

// Get the load on the north column after one billion spin cycles.
fn part2(input: AocInput) -> usize {
    let mut platform = Platform::from_input(input);
    platform.spin_cycle(1000000000)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Platform(Grid<Tile>);

impl Platform {
    fn from_input(input: AocInput) -> Self {
        let grid = GridBuilder::from_input(input)
            .map(|_, b| match b {
                b'O' => Tile::RoundRock,
                b'#' => Tile::SquareRock,
                b'.' => Tile::Ground,
                _ => unreachable!(),
            })
            .build();

        Self(grid)
    }

    fn calculate_load(&self) -> usize {
        self.0
            .cells()
            .filter_map(|(pos, tile)| {
                (*tile == Tile::RoundRock).then_some(self.0.height() - pos.row())
            })
            .sum()
    }

    fn spin_cycle(&mut self, cycles: usize) -> usize {
        let directions = [
            PointDiff::UP,
            PointDiff::LEFT,
            PointDiff::DOWN,
            PointDiff::RIGHT,
        ];

        let mut seen = HashMap::new();
        let mut loads = Vec::new();
        for cycle in 0..cycles {
            let load = self.calculate_load();
            loads.push(load);
            println!("{cycle}: {load}");
            if let Some(previous) = seen.insert(self.clone(), cycle) {
                println!("Found cycle between {previous} and {cycle}");
                let len = cycle - previous;
                let target = previous + ((cycles - previous) % len);
                return loads[target];
            }

            for dir in &directions {
                self.tilt(*dir);
            }
        }

        *loads.last().unwrap()
    }

    fn tilt(&mut self, dir: PointDiff) {
        let rect = self.0.bounding_rect();
        let points: Box<dyn Iterator<Item = Point>> = match dir {
            PointDiff::UP => Box::new(rect.points()),
            PointDiff::DOWN => Box::new(rect.points().rev()),
            PointDiff::LEFT => Box::new(rect.points_by_col()),
            PointDiff::RIGHT => Box::new(rect.points_by_col().rev()),
            _ => unreachable!(),
        };

        for pos in points {
            if self.0[pos] == Tile::RoundRock {
                self.move_rock(pos, dir);
            }
        }
    }

    fn move_rock(&mut self, mut pos: Point, dir: PointDiff) {
        loop {
            let Some(next) = self.0.add_point(pos, dir) else {
                break;
            };

            if self.0[next] != Tile::Ground {
                break;
            }

            self.0[next] = Tile::RoundRock;
            self.0[pos] = Tile::Ground;
            pos = next;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Ground,
    RoundRock,
    SquareRock,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(136, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(64, part2(AocInput::from_sample()));
    }
}
