use std::path::Path;

use crate::{get_input, FileInput};

use super::Grid;

pub struct GridBuilder<T, I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    input: I,
    transform: Box<dyn Fn(u8) -> T>,
    extend: Option<(usize, usize, u8)>,
}

impl GridBuilder<u8, FileInput, String> {
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        Self {
            input: get_input(path),
            transform: Box::new(|byte| byte),
            extend: None,
        }
    }
}

impl<I, S> GridBuilder<u8, I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    pub fn from_lines(input: I) -> Self {
        Self {
            input,
            transform: Box::new(|byte| byte),
            extend: None,
        }
    }
}

impl<T, I, S> GridBuilder<T, I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    pub fn map<U>(self, transform: impl Fn(u8) -> U + Clone + 'static) -> GridBuilder<U, I, S> {
        GridBuilder {
            input: self.input,
            transform: Box::new(transform),
            extend: self.extend,
        }
    }

    /// Extends the grid in all directions by the specified amount.
    /// Use with 0 to make non-uniform input uniform.
    pub fn extend(self, width: usize, height: usize, value: u8) -> Self {
        Self {
            extend: Some((width, height, value)),
            ..self
        }
    }

    pub fn build(self) -> Grid<T> {
        let (extend_width, extend_height, extend_value) = self.extend.unwrap_or((0, 0, 0));
        let mut grid: Vec<_> = (0..extend_height).map(|_| Vec::new()).collect();

        grid.extend(self.input.map(|line| {
            let mut row: Vec<_> = (0..extend_width)
                .map(|_| (self.transform)(extend_value))
                .collect();

            row.extend(line.as_ref().bytes().map(self.transform.as_ref()));
            row.extend((0..extend_width).map(|_| (self.transform)(extend_value)));
            row
        }));

        if self.extend.is_some() {
            grid.extend((0..extend_height).map(|_| Vec::new()));
            let max_width = grid.iter().map(|row| row.len()).max().unwrap();
            for row in &mut grid {
                row.resize_with(max_width, || (self.transform)(extend_value))
            }
        }

        Grid(grid)
    }
}
