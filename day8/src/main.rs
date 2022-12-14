// https://adventofcode.com/2022/day/8
//
// Not super happy with this one. I think a cleaner solution is possible, maybe with some more code
// reuse between both parts, but I don't feel like spending more time on this.
use aoc::grid::grid_from_file;
use aoc::grid::Grid;
use aoc::grid::Point;
use aoc::grid::PointDiff;

fn main() {
    const PATH: &str = "input/day8.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

fn part1(path: &str) -> u32 {
    let mut grid = TreeGrid::parse(path);
    grid.count_visible()
}

fn part2(path: &str) -> u32 {
    let grid = TreeGrid::parse(path);
    grid.get_max_scenic_score()
}

#[derive(Clone, Copy)]
struct Tree {
    height: u8,
    visible: bool,
}

impl Tree {
    fn new(height: u8) -> Self {
        Tree {
            height,
            visible: false,
        }
    }
}

struct TreeGrid(Grid<Tree>);

impl TreeGrid {
    pub fn parse(path: &str) -> Self {
        Self(grid_from_file(path).map(|cell| Tree::new(cell - b'0')))
    }

    pub fn count_visible(&mut self) -> u32 {
        let edge: Vec<_> = self.0.edge_cells().map(|(point, _)| point).collect();
        for point in edge {
            if point.row() == 0 {
                self.check_visible(point, PointDiff::DOWN);
            } else if point.row() == self.0.height() - 1 {
                self.check_visible(point, PointDiff::UP);
            }

            if point.col() == 0 {
                self.check_visible(point, PointDiff::RIGHT);
            } else if point.col() == self.0.width() - 1 {
                self.check_visible(point, PointDiff::LEFT);
            }
        }

        self.0
            .cells()
            .map(|(_, tree)| u32::from(tree.visible))
            .sum()
    }

    pub fn get_max_scenic_score(&self) -> u32 {
        self.0
            .cells()
            .map(|(point, _)| self.get_scenic_score(point))
            .max()
            .unwrap()
    }

    fn check_visible(&mut self, start: Point, dir: PointDiff) {
        let mut max_height = None;
        for tree in self.0.scan_mut(start, dir) {
            if max_height.is_none() || max_height.unwrap() < tree.height {
                tree.visible = true;
                max_height = Some(tree.height);
            }
        }
    }

    fn get_scenic_score(&self, point: Point) -> u32 {
        PointDiff::STRAIGHT_NEIGHBORS
            .iter()
            .map(|dir| self.get_viewing_distance(point, *dir))
            .product()
    }

    fn get_viewing_distance(&self, point: Point, dir: PointDiff) -> u32 {
        let height = self.0[point].height;

        let mut distance = 0;
        for tree in self.0.scan(point, dir).skip(1) {
            distance += 1;
            if tree.height >= height {
                break;
            }
        }

        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "../input/sample/day8.txt";

    #[test]
    fn test_part1() {
        println!("{:?}", std::env::current_dir().unwrap());
        assert_eq!(21, part1(PATH));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, part2(PATH));
    }
}
