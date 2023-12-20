// https://adventofcode.com/2023/day/17

use std::collections::HashSet;

use aoc::{
    dijkstra::{path_from_info, shortest_paths, Graph},
    grid::{Grid, GridBuilder, Point, PointDiff, Rotation},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find the path that minimizes heat loss.
fn part1(input: AocInput) -> usize {
    let map = TrafficMap::from_input(input);
    map.min_heat_loss()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct TrafficMap(Grid<u8>);

impl TrafficMap {
    fn from_input(input: AocInput) -> Self {
        let grid = GridBuilder::from_input(input).map(|_, b| b - b'0').build();
        Self(grid)
    }

    fn min_heat_loss(&self) -> usize {
        let start = MapVertex {
            pos: Point::default(),
            dir: PointDiff::RIGHT,
            steps: 0,
        };

        let info = shortest_paths(self, &start);
        let dest = self.0.bounding_rect().bottom_right();

        // Of all the vertices matching the destination position, find the one with the smallest
        // heat loss.
        let (dest, dest_info) = info
            .iter()
            .filter(|(key, _)| key.pos == dest)
            .min_by_key(|(_, value)| value.distance)
            .unwrap();

        // Get the path and draw it.
        let path = path_from_info(&info, dest);
        for (row_index, row) in self.0.rows().enumerate() {
            for (col, cell) in row.iter().enumerate() {
                if let Some(v) = path.iter().find(|v| v.pos == Point::new(row_index, col)) {
                    print!("{}", v.dir.get_dir_char().unwrap())
                } else {
                    print!("{cell}")
                }
            }

            println!();
        }

        dest_info.distance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MapVertex {
    pos: Point,
    dir: PointDiff,
    steps: usize,
}

impl Graph<MapVertex> for TrafficMap {
    fn vertices(&self) -> HashSet<MapVertex> {
        self.0
            .bounding_rect()
            .points()
            .flat_map(|pos| {
                PointDiff::STRAIGHT_NEIGHBORS
                    .iter()
                    .flat_map(move |&dir| (0..4).map(move |steps| MapVertex { pos, dir, steps }))
            })
            .collect()
    }

    fn neighbors(&self, v: &MapVertex) -> Vec<(MapVertex, usize)> {
        let mut result = Vec::new();
        if v.steps < 3 {
            if let Some(pos) = self.0.add_point(v.pos, v.dir) {
                result.push((
                    MapVertex {
                        pos,
                        dir: v.dir,
                        steps: v.steps + 1,
                    },
                    self.0[pos] as usize,
                ));
            }
        }

        for dir in [v.dir.rotate(Rotation::Left), v.dir.rotate(Rotation::Right)] {
            if let Some(pos) = self.0.add_point(v.pos, dir) {
                result.push((MapVertex { pos, dir, steps: 1 }, self.0[pos] as usize));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(102, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
