#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use aoc::dijkstra::Graph;
use aoc::get_input;
use aoc::sliding_window::HasSlidingWindow;
use std::str::FromStr;

fn main() {
    const PATH: &str = "input/day12.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

fn part1(path: &str) -> usize {
    let map = HeightMap::parse(path);
    map.shortest_path()
}

fn part2(path: &str) -> usize {
    let map = HeightMap::parse(path);
    map.shortest_path_from_lowest()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Point(isize, isize);

#[derive(Clone)]
struct HeightMap {
    map: Vec<Vec<u8>>,
    start: Point,
    end: Point,
    invert: bool,
}

impl HeightMap {
    fn parse(path: &str) -> Self {
        let mut start = Point::default();
        let mut end = Point::default();
        let map = get_input(path)
            .enumerate()
            .map(|(row, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(col, ch)| match ch {
                        b'S' => {
                            start = Point(row as isize, col as isize);
                            0
                        }
                        b'E' => {
                            end = Point(row as isize, col as isize);
                            25
                        }
                        ch => ch - b'a',
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            start,
            end,
            invert: false,
        }
    }

    fn shortest_path(&self) -> usize {
        let path = aoc::dijkstra::shortest_path(self, &self.end, &self.start);
        let mut path_map = self.clone();
        for row in path_map.map.iter_mut() {
            for col in row.iter_mut() {
                *col = b'.';
            }
        }

        for p in path.as_slice().sliding_window(2) {
            *path_map.get_mut(&p[0]).unwrap() = match (p[1].0 - p[0].0, p[1].1 - p[0].1) {
                (-1, 0) => b'^',
                (1, 0) => b'v',
                (0, -1) => b'<',
                (0, 1) => b'>',
                _ => unreachable!(),
            }
        }

        *path_map.get_mut(path.last().unwrap()).unwrap() = b'E';
        for row in path_map.map.iter_mut() {
            for col in row.iter_mut() {
                print!("{}", *col as char)
            }

            println!();
        }

        // Resulting path includes start which we shouldn't count.
        path.len() - 1
    }

    fn shortest_path_from_lowest(&self) -> usize {
        // Invert the height check for neighbors because we're walking from the end to all points.
        let mut map = self.clone();
        map.invert = true;

        // Calculate all paths starting at the end.
        let info = aoc::dijkstra::shortest_paths(&map, &self.end);

        // See which is the shortest from a square of height 0.
        let mut min = usize::MAX;
        for (row, values) in self.map.iter().enumerate() {
            for (col, height) in values.iter().enumerate() {
                if *height == 0 {
                    let p = Point(row as isize, col as isize);
                    let distance = info[&p].distance;
                    if distance < min {
                        min = distance;
                    }
                }
            }
        }

        min
    }

    fn get(&self, p: &Point) -> Option<u8> {
        self.map.get(p.0 as usize)?.get(p.1 as usize).copied()
    }

    fn get_mut(&mut self, p: &Point) -> Option<&mut u8> {
        self.map.get_mut(p.0 as usize)?.get_mut(p.1 as usize)
    }
}

impl Graph<Point> for HeightMap {
    fn vertices(&self) -> std::collections::HashSet<Point> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(row, items)| {
                items
                    .iter()
                    .enumerate()
                    .map(move |(col, _)| Point(row as isize, col as isize))
            })
            .collect()
    }

    fn neighbors(&self, v: &Point) -> Vec<Point> {
        let height = self.get(v).unwrap();
        let mut result = Vec::new();
        for offset in [-1, 1] {
            let neighbor = Point(v.0 + offset, v.1);
            if let Some(candidate) = self.get(&neighbor) {
                // I hate this.
                if height <= candidate + 1 {
                    result.push(neighbor)
                }
            }
        }

        for offset in [-1, 1] {
            let neighbor = Point(v.0, v.1 + offset);
            if let Some(candidate) = self.get(&neighbor) {
                if height <= candidate + 1 {
                    result.push(neighbor)
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "../input/sample/day12.txt";

    #[test]
    fn test_part1() {
        assert_eq!(31, part1(PATH));
    }

    #[test]
    fn test_part2() {
        assert_eq!(29, part2(PATH));
    }
}
