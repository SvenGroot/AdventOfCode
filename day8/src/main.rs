// https://adventofcode.com/2022/day/8
//
// Not super happy with this one. I think a cleaner solution is possible, maybe with some more code
// reuse between both parts, but I don't feel like spending more time on this.
use aoc::get_input;

fn main() {
    const PATH: &str = "input/day8.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

fn part1(path: &str) -> u32 {
    let mut grid = Grid::parse(path);
    grid.count_visible()
}

fn part2(path: &str) -> u32 {
    let grid = Grid::parse(path);
    grid.get_max_scenic_score()
}

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

struct Grid(Vec<Vec<Tree>>);

impl Grid {
    pub fn parse(path: &str) -> Self {
        let grid = get_input(path)
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|ch| Tree::new(ch - b'0'))
                    .collect()
            })
            .collect();

        Self(grid)
    }

    pub fn count_visible(&mut self) -> u32 {
        for (col, col_dir) in [(self.0[0].len() - 1, -1), (0, 1)] {
            for row in 0..self.0.len() {
                self.check_visible(row, col, 0, col_dir);
            }
        }

        for (row, row_dir) in [(self.0.len() - 1, -1), (0, 1)] {
            for col in 0..self.0[0].len() {
                self.check_visible(row, col, row_dir, 0);
            }
        }

        self.0
            .iter()
            .map(|row| row.iter().map(|tree| u32::from(tree.visible)).sum::<u32>())
            .sum()
    }

    pub fn get_max_scenic_score(&self) -> u32 {
        let mut max = 0;
        // No need to check the edges; they're all 0.
        for row in 1..self.0.len() - 1 {
            for col in 1..self.0[row].len() - 1 {
                let score = self.get_scenic_score(row, col);
                if score > max {
                    max = score;
                }
            }
        }

        max
    }

    fn check_visible(&mut self, row: usize, col: usize, row_dir: isize, col_dir: isize) {
        let mut row: isize = row.try_into().unwrap();
        let mut col: isize = col.try_into().unwrap();
        let mut max_height = None;
        while row >= 0
            && (row as usize) < self.0.len()
            && col >= 0
            && (col as usize) < self.0[row as usize].len()
        {
            let tree = &mut self.0[row as usize][col as usize];
            if max_height.is_none() || max_height.unwrap() < tree.height {
                tree.visible = true;
                max_height = Some(tree.height);
            }

            row += row_dir;
            col += col_dir;
        }
    }

    fn get_scenic_score(&self, row: usize, col: usize) -> u32 {
        let mut score = 1;
        for col_dir in [-1, 1] {
            score *= self.get_viewing_distance(row, col, 0, col_dir);
        }

        for row_dir in [-1, 1] {
            score *= self.get_viewing_distance(row, col, row_dir, 0);
        }

        score
    }

    fn get_viewing_distance(&self, row: usize, col: usize, row_dir: isize, col_dir: isize) -> u32 {
        let height = self.0[row][col].height;
        let mut row: isize = row.try_into().unwrap();
        let mut col: isize = col.try_into().unwrap();
        row += row_dir;
        col += col_dir;

        let mut distance = 0;
        while row >= 0
            && (row as usize) < self.0.len()
            && col >= 0
            && (col as usize) < self.0[row as usize].len()
        {
            distance += 1;
            if self.0[row as usize][col as usize].height >= height {
                break;
            }

            row += row_dir;
            col += col_dir;
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
