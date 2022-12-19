use std::path::Path;

use aoc::{
    aoc_input, get_input,
    grid::{Grid, Point, PointDiff},
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let mut sim = SandSimulator::from_file(path, false);
    println!("{}", sim.grid);
    let count = sim.simulate();
    println!("{}", sim.grid);
    count
}

fn part2(path: impl AsRef<Path>) -> usize {
    let mut sim = SandSimulator::from_file(path, true);
    //println!("{}", sim.grid);
    let count = sim.simulate();
    println!("{}", sim.grid);
    assert!(sim.grid[SandSimulator::ORIGIN - sim.offset] == 'o');
    count
}

struct SandSimulator {
    grid: Grid<char>,
    offset: Point,
}

impl SandSimulator {
    const ORIGIN: Point = Point::new(0, 500);

    fn from_file(path: impl AsRef<Path>, with_floor: bool) -> Self {
        let mut lines = Self::read_lines(path);
        let mut max_row = lines
            .iter()
            .flatten()
            .max_by_key(|p| p.row())
            .unwrap()
            .row();
        let mut min_col = lines
            .iter()
            .flatten()
            .min_by_key(|p| p.col())
            .unwrap()
            .col();
        let mut max_col = lines
            .iter()
            .flatten()
            .max_by_key(|p| p.col())
            .unwrap()
            .col();

        if with_floor {
            // Widen the grid so the offset is 0, and by the same amount on the right.
            max_col += min_col;
            min_col = 0;
            max_row += 2;
            lines.push(vec![
                Point::new(max_row, min_col),
                Point::new(max_row, max_col),
            ]);
        }

        let offset = Point::new(0, min_col);

        let mut grid = Grid::new(
            (max_row + 1).try_into().unwrap(),
            (max_col - min_col + 1).try_into().unwrap(),
            '.',
        );

        // Draw the rock formations.
        for path in lines {
            for pair in path.as_slice().windows(2) {
                for p in pair[0].line_to(pair[1]).unwrap() {
                    grid[p - offset] = '#';
                }
            }
        }

        // Mark the sand origin.
        grid[Self::ORIGIN - offset] = '+';

        Self { grid, offset }
    }

    fn read_lines(path: impl AsRef<Path>) -> Vec<Vec<Point>> {
        get_input(path)
            .map(|line| line.split(" -> ").map(|p| p.parse().unwrap()).collect())
            .collect()
    }

    pub fn simulate(&mut self) -> usize {
        let mut count = 0;
        while self.simulate_unit().is_some() {
            count += 1;
            if self.grid[Self::ORIGIN - self.offset] != '+' {
                break;
            }
        }

        count
    }

    fn simulate_unit(&mut self) -> Option<()> {
        let mut sand = Self::ORIGIN - self.offset;
        loop {
            let mut moved = false;
            for dir in [PointDiff::DOWN, PointDiff::DOWN_LEFT, PointDiff::DOWN_RIGHT] {
                let next = sand.add_diff(dir)?;
                let content = self.grid.get(next)?;
                if *content == '.' {
                    sand = next;
                    moved = true;
                    break;
                }
            }

            // Came to rest.
            if !moved {
                self.grid[sand] = 'o';
                return Some(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(24, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(93, part2(aoc_sample_input()));
    }
}
