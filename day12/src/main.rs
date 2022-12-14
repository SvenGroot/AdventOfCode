use aoc::aoc_input;
use aoc::dijkstra::Graph;
use aoc::grid::{grid_from_file, Grid, Point, PointDiff};
use std::path::Path;

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let map = HeightMap::parse(path);
    map.shortest_path()
}

fn part2(path: impl AsRef<Path>) -> usize {
    let map = HeightMap::parse(path);
    map.shortest_path_from_lowest()
}

#[derive(Clone)]
struct HeightMap {
    map: Grid<u8>,
    start: Point,
    end: Point,
}

impl HeightMap {
    fn parse(path: impl AsRef<Path>) -> Self {
        let mut start = Point::default();
        let mut end = Point::default();
        let mut map = grid_from_file(path);
        for (point, height) in map.cells_mut() {
            match height {
                b'S' => {
                    start = point;
                    *height = b'a';
                }
                b'E' => {
                    end = point;
                    *height = b'z';
                }
                _ => (),
            }
        }

        Self { map, start, end }
    }

    fn shortest_path(&self) -> usize {
        // We're getting the path in reverse because the height check is reversed to make part 2 easier.
        let path = aoc::dijkstra::shortest_path(self, &self.end, &self.start);
        let mut path_map = Grid::new(
            self.map.height().try_into().unwrap(),
            self.map.width().try_into().unwrap(),
            '.',
        );

        for p in path.as_slice().windows(2) {
            // The path is backwards!
            *path_map.get_mut(p[1]).unwrap() = match p[0].diff(p[1]).unwrap() {
                PointDiff::UP => '^',
                PointDiff::DOWN => 'v',
                PointDiff::LEFT => '<',
                PointDiff::RIGHT => '>',
                _ => unreachable!(),
            }
        }

        path_map[path[0]] = 'E';
        println!("{}", path_map);

        // Resulting path includes start which we shouldn't count.
        path.len() - 1
    }

    fn shortest_path_from_lowest(&self) -> usize {
        // Calculate all paths starting at the end (height check is reversed to allow this).
        let info = aoc::dijkstra::shortest_paths(self, &self.end);

        // See which is the shortest from a square of height 0.
        self.map
            .cells()
            .filter_map(|(point, height)| (*height == b'a').then_some(info[&point].distance))
            .min()
            .unwrap()
    }

    fn get(&self, p: Point) -> Option<u8> {
        self.map.get(p).copied()
    }
}

impl Graph<Point> for HeightMap {
    fn vertices(&self) -> std::collections::HashSet<Point> {
        self.map.cells().map(|(point, _)| point).collect()
    }

    fn neighbors(&self, v: &Point) -> Vec<(Point, usize)> {
        let height = self.get(*v).unwrap();
        self.map
            .straight_neighbors(*v)
            .filter_map(|nb| {
                // This is backwards to make part 2 easier.
                (height <= self.map[nb] + 1).then_some((nb, 1))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(31, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(29, part2(aoc_sample_input()));
    }
}
