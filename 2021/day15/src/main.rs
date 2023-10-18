// https://adventofcode.com/2021/day/15

use aoc::{
    dijkstra::{shortest_path, Graph},
    grid::{Grid, GridBuilder, Point, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find the path with the lowest total risk level.
fn part1(input: AocInput) -> usize {
    let map = CeilingMap::from_input(input);
    map.lowest_risk()
}

// Find the path with the lowest total risk level in the extended map.
fn part2(input: AocInput) -> usize {
    let mut map = CeilingMap::from_input(input);
    map.extend();
    map.lowest_risk()
}

struct CeilingMap(Grid<u8>);

impl CeilingMap {
    fn from_input(input: AocInput) -> Self {
        Self(GridBuilder::from_input(input).numbers().build())
    }

    fn extend(&mut self) {
        let width = self.0.width();
        let height = self.0.height();
        let up = PointDiff::UP * height as isize;
        let left = PointDiff::LEFT * width as isize;
        self.0.grow_with(height * 5, width * 5, 0);
        for pos in self.0.bounding_rect().points() {
            if pos.row() >= height {
                self.0[pos] = self.0[pos + up] + 1;
            } else if pos.col() >= width {
                self.0[pos] = self.0[pos + left] + 1;
            }

            if self.0[pos] == 10 {
                self.0[pos] = 1;
            }
        }
    }

    fn lowest_risk(&self) -> usize {
        let source = Point::default();
        let dest = self.0.bounding_rect().bottom_right();

        // This is just the shortest path using the "risk levels" as the weights.
        let path = shortest_path(self, &source, &dest);

        // Sum of all risk levels except the first.
        path.iter().skip(1).map(|pos| self.0[*pos] as usize).sum()
    }
}

impl Graph<Point> for CeilingMap {
    fn vertices(&self) -> std::collections::HashSet<Point> {
        self.0.bounding_rect().points().collect()
    }

    fn neighbors(&self, v: &Point) -> Vec<(Point, usize)> {
        // Use the value (risk level) in the neighboring cell as the edge weight.
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
        assert_eq!(315, part2(AocInput::from_sample()));
    }
}
