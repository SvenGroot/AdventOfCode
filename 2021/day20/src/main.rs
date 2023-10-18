// https://adventofcode.com/2021/day/20

use std::fmt::Display;

use aoc::{
    bitfield::BitField,
    grid::{Grid, GridBuilder, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Enhance the image twice. Count lit pixels.
fn part1(input: AocInput) -> usize {
    let mut img = Image::from_input(input);
    img.enhance(2);
    img.count_lit()
}

// Enhance the image 50 times. Count lit pixels.
fn part2(input: AocInput) -> usize {
    let mut img = Image::from_input(input);
    img.enhance(50);
    img.count_lit()
}

struct Image {
    enhancement: Vec<bool>,
    grid: Grid<bool>,
    // The value of cells beyond the edges of the grid.
    overflow_val: bool,
}

impl Image {
    fn from_input(mut input: AocInput) -> Self {
        let enhancement = input.next().unwrap().bytes().map(|b| b == b'#').collect();
        input.next().unwrap();
        let grid = GridBuilder::from_input(input)
            .map(|b| b == b'#')
            .extend(100, 100, b'.')
            .build();

        Self {
            enhancement,
            grid,
            overflow_val: false,
        }
    }

    fn enhance(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.run_enhancement();
            // println!("{self}");
        }
    }

    fn count_lit(&self) -> usize {
        self.grid.cells().filter(|cell| *cell.1).count()
    }

    fn run_enhancement(&mut self) {
        const SURROUNDING: [PointDiff; 9] = [
            PointDiff::DOWN_RIGHT,
            PointDiff::DOWN,
            PointDiff::DOWN_LEFT,
            PointDiff::RIGHT,
            PointDiff::ZERO,
            PointDiff::LEFT,
            PointDiff::UP_RIGHT,
            PointDiff::UP,
            PointDiff::UP_LEFT,
        ];

        let mut enhanced = Grid::new(self.grid.height(), self.grid.width(), false);
        for (pos, cell) in enhanced.cells_mut() {
            let mut index = BitField::default();

            // Don't use pos.neighbors because we want to use the overflow value for col/row below 0.
            for (bit_index, value) in SURROUNDING
                .iter()
                .map(|dir| pos.add_diff(*dir).and_then(|nb| self.grid.get(nb)))
                .enumerate()
            {
                index = index.set(bit_index, *value.unwrap_or(&self.overflow_val));
            }

            *cell = self.enhancement[index.value() as usize];
        }

        self.grid = enhanced;
        if self.overflow_val {
            self.overflow_val = self.enhancement[self.enhancement.len() - 1];
        } else {
            self.overflow_val = self.enhancement[0];
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let subgrid = self.grid.sub_grid_where(|item| *item);
        for row in subgrid.rows() {
            for cell in row {
                write!(f, "{}", if *cell { '#' } else { '.' })?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3351, part2(AocInput::from_sample()));
    }
}
