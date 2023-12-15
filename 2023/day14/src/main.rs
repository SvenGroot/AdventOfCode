// https://adventofcode.com/2023/day/14

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

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

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

    fn tilt(&mut self, dir: PointDiff) {
        // This order only works for PointDiff::UP.
        for pos in self.0.bounding_rect().points() {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
