// https://adventofcode.com/2021/day/9

use aoc::{
    grid::{Grid, GridBuilder, Point},
    input::AocInput,
    iterator::IteratorExt,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Sum the risk level of each low point.
fn part1(input: AocInput) -> usize {
    let map = HeightMap::from_input(input);
    map.get_low_points().map(|value| value.1 as usize + 1).sum()
}

// Find the three largest basins.
fn part2(input: AocInput) -> usize {
    let mut map = HeightMap::from_input(input);
    let lows = map.get_low_points().map(|value| value.0).into_vec();
    let mut basins = lows.into_iter().map(|pos| map.get_basin(pos)).into_vec();
    basins.sort();
    let len = basins.len();
    basins[len - 1] * basins[len - 2] * basins[len - 3]
}

struct Tile {
    height: u8,
    basin_low: Option<Point>,
}

struct HeightMap(Grid<Tile>);

impl HeightMap {
    fn from_input(input: AocInput) -> Self {
        let grid = GridBuilder::from_input(input)
            .map(|byte| Tile {
                height: byte - b'0',
                basin_low: None,
            })
            .build();

        Self(grid)
    }

    fn get_low_points(&self) -> impl Iterator<Item = (Point, u8)> + '_ {
        self.0.cells().filter_map(|(pos, cell)| {
            self.0
                .straight_neighbors(pos)
                .all(|nb| self.0[nb].height > cell.height)
                .then_some((pos, cell.height))
        })
    }

    fn get_basin(&mut self, pos: Point) -> usize {
        let tile = &mut self.0[pos];
        if let Some(basin_low) = tile.basin_low {
            // Sanity check this wasn't part of another basin.
            assert!(basin_low == pos);
            return 0;
        }

        tile.basin_low = Some(pos);
        let mut size = 1;

        // Walk surrounding tiles that aren't 9 (those aren't part of any basin).
        for nb in self.0.straight_neighbors(pos) {
            if self.0[nb].height != 9 {
                size += self.get_basin(nb);
            }
        }

        size
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
        assert_eq!(1134, part2(AocInput::from_sample()));
    }
}
