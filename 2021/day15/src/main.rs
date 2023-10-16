// https://adventofcode.com/2021/day/15

use aoc::{
    dijkstra::{shortest_path, Graph},
    grid::{Grid, GridBuilder, Point},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find the path with the lowest total risk level.
fn part1(input: AocInput) -> usize {
    let map = CeilingMap::from_input(input);
    let source = Point::default();
    let dest = map.0.bounding_rect().bottom_right();
    let path = shortest_path(&map, &source, &dest);
    path.iter().skip(1).map(|pos| map.0[*pos] as usize).sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct CeilingMap(Grid<u8>);

impl CeilingMap {
    fn from_input(input: AocInput) -> Self {
        Self(GridBuilder::from_input(input).numbers().build())
    }
}

impl Graph<Point> for CeilingMap {
    fn vertices(&self) -> std::collections::HashSet<Point> {
        self.0.bounding_rect().points().collect()
    }

    fn neighbors(&self, v: &Point) -> Vec<(Point, usize)> {
        self.0
            .straight_neighbors(*v)
            .map(|nb| (nb, self.0[nb] as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(40, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
