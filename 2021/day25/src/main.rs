// https://adventofcode.com/2021/day/25

use std::fmt::Display;

use aoc::{
    grid::{Grid, GridBuilder, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
}

// After how many steps do the sea cucumbers stop moving.
fn part1(input: AocInput) -> usize {
    let mut map = SeaCucumberMap::from_input(input);
    let mut steps = 1;
    while map.move_herds() {
        steps += 1
    }

    println!("{map}");
    steps
}

struct SeaCucumberMap(Grid<Option<PointDiff>>);

impl SeaCucumberMap {
    fn from_input(input: AocInput) -> Self {
        Self(
            GridBuilder::from_input(input)
                .map(|ch| PointDiff::from_char(ch, 0, b'>', b'v', 0))
                .build(),
        )
    }

    fn move_herds(&mut self) -> bool {
        // Not short-circuiting
        self.move_herd(PointDiff::RIGHT) | self.move_herd(PointDiff::DOWN)
    }

    fn move_herd(&mut self, herd_dir: PointDiff) -> bool {
        let mut has_move = false;
        let mut new_grid = self.0.clone();
        for (pos, dir) in self.0.cells() {
            if *dir == Some(herd_dir) {
                let next = self.0.add_point_wrapped(pos, herd_dir);
                if self.0[next].is_none() {
                    new_grid[pos] = None;
                    new_grid[next] = Some(herd_dir);
                    has_move = true;
                }
            }
        }

        self.0 = new_grid;
        has_move
    }
}

impl Display for SeaCucumberMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .write_mapped(f, |dir| dir.map_or('.', |dir| dir.get_dir_char().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(58, part1(AocInput::from_sample()));
    }
}
