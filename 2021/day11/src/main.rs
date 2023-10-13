// https://adventofcode.com/2021/day/11

use std::collections::HashSet;

use aoc::{
    grid::{Grid, GridBuilder},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    let mut map = OctopusMap::from_input(input);
    map.simulate(100)
}

fn part2(input: AocInput) -> usize {
    let mut map = OctopusMap::from_input(input);
    map.simulate_until_sync()
}

struct OctopusMap(Grid<u8>);

impl OctopusMap {
    fn from_input(input: AocInput) -> Self {
        Self(GridBuilder::from_input(input).numbers().build())
    }

    fn simulate(&mut self, steps: usize) -> usize {
        (0..steps).map(|_| self.simulate_step()).sum()
    }

    fn simulate_until_sync(&mut self) -> usize {
        let mut step = 1;
        let count = self.0.width() * self.0.height();
        while self.simulate_step() != count {
            step += 1;
        }

        step
    }

    fn simulate_step(&mut self) -> usize {
        // First increment all the octopodes.
        for (_, energy) in self.0.cells_mut() {
            *energy = (*energy + 1) % 10;
        }

        // Then check neighbors of ones that flashed.
        let mut has_new = true;
        let mut visited = HashSet::new();
        while has_new {
            has_new = false;
            for pos in self.0.bounding_rect().points() {
                if self.0[pos] == 0 && visited.insert(pos) {
                    for nb in self.0.all_neighbors(pos) {
                        if self.0[nb] != 0 {
                            self.0[nb] = (self.0[nb] + 1) % 10;
                            has_new |= self.0[nb] == 0;
                        }
                    }
                }
            }
        }

        // Count the flashing octopodes
        self.0.cells().filter(|(_, energy)| **energy == 0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1656, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(195, part2(AocInput::from_sample()));
    }
}
