// https://adventofcode.com/2021/day/9

use aoc::{
    grid::{Grid, GridBuilder},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Sum the risk level of each low point.
fn part1(input: AocInput) -> usize {
    let map = HeightMap::from_input(input);
    map.get_low_points().map(|value| value as usize + 1).sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct HeightMap(Grid<u8>);

impl HeightMap {
    fn from_input(input: AocInput) -> Self {
        let grid = GridBuilder::from_input(input)
            .map(|byte| byte - b'0')
            .build();

        Self(grid)
    }

    fn get_low_points(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.cells().filter_map(|(pos, cell)| {
            self.0
                .straight_neighbors(pos)
                .all(|nb| self.0[nb] > *cell)
                .then_some(*cell)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(15, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
