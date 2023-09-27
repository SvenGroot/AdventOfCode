// https://adventofcode.com/2022/day/18
#[macro_use]
extern crate text_io;

use std::path::Path;

use aoc::{
    aoc_input, get_input,
    grid3d::{Grid3D, Point3D},
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let grid = load_grid(path);

    grid.cells()
        .map(|(point, &occupied)| {
            if occupied {
                grid.straight_neighbors(point)
                    .filter(|nb| !grid[*nb])
                    .count()
            } else {
                0
            }
        })
        .sum()
}

fn part2(path: impl AsRef<Path>) -> usize {
    let mut grid = load_grid(path).map(|&value| if value { State::Lava } else { State::Unknown });
    expand_steam(&mut grid);

    grid.cells()
        .map(|(point, &state)| {
            if state == State::Lava {
                grid.straight_neighbors(point)
                    .filter(|&nb| grid[nb] == State::Outside)
                    .count()
            } else {
                0
            }
        })
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Unknown,
    Lava,
    Outside,
}

fn load_grid(path: impl AsRef<Path>) -> Grid3D<bool> {
    let mut grid = Grid3D::new(
        30.try_into().unwrap(),
        30.try_into().unwrap(),
        30.try_into().unwrap(),
        false,
    );

    for line in get_input(path) {
        let x: usize;
        let y: usize;
        let z: usize;
        scan!(line.bytes() => "{},{},{}", x, y, z);
        // Make sure none are on the edge so all neighbors are counted.
        grid[Point3D::new(x + 1, y + 1, z + 1)] = true;
    }
    grid
}

fn expand_steam(grid: &mut Grid3D<State>) {
    let mut to_check = vec![Point3D::new(0, 0, 0)];
    while let Some(point) = to_check.pop() {
        grid[point] = State::Outside;
        to_check.extend(
            grid.straight_neighbors(point)
                .filter(|&nb| grid[nb] == State::Unknown),
        );
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(64, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(58, part2(aoc_sample_input()));
    }
}
