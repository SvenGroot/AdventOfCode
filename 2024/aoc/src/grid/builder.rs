use ndarray::prelude::*;

use crate::input::AocInput;

use super::Grid;
use super::Point;

pub struct GridBuilder<T, I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    input: I,
    transform: Box<dyn FnMut(Point, u8) -> T>,
    extend: Option<(usize, usize, u8)>,
}

impl GridBuilder<u8, AocInput, String> {
    pub fn from_input(input: AocInput) -> Self {
        Self {
            input,
            transform: Box::new(|_, byte| byte),
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
            transform: Box::new(|_, byte| byte),
            extend: None,
        }
    }
}

impl<T, I, S> GridBuilder<T, I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    pub fn map<U>(
        self,
        transform: impl FnMut(Point, u8) -> U + Clone + 'static,
    ) -> GridBuilder<U, I, S> {
        GridBuilder {
            input: self.input,
            transform: Box::new(transform),
            extend: self.extend,
        }
    }

    pub fn numbers(self) -> GridBuilder<u8, I, S> {
        self.map(|_, byte| byte - b'0')
    }

    pub fn chars(self) -> GridBuilder<char, I, S> {
        self.map(|_, byte| byte as char)
    }

    /// Extends the grid in all directions by the specified amount.
    /// Use with 0 to make non-uniform input uniform.
    pub fn extend(self, width: usize, height: usize, value: u8) -> Self {
        Self {
            extend: Some((width, height, value)),
            ..self
        }
    }

    pub fn build(mut self) -> Grid<T> {
        let (extend_width, extend_height, extend_value) = self.extend.unwrap_or((0, 0, 0));
        let mut grid: Vec<_> = (0..extend_height).map(|_| Vec::new()).collect();

        let mut row_index = extend_height;
        grid.extend(self.input.map(|line| {
            let mut row: Vec<_> = (0..extend_width)
                .map(|col| (self.transform)(Point::new(row_index, col), extend_value))
                .collect();

            row.extend(line.as_ref().bytes().enumerate().map(|(col, value)| {
                (self.transform)(Point::new(row_index, col + extend_width), value)
            }));

            let col_offset = extend_width + line.as_ref().len();
            row.extend((0..extend_width).map(|col| {
                (self.transform)(Point::new(row_index, col + col_offset), extend_value)
            }));
            row_index += 1;
            row
        }));

        if self.extend.is_some() {
            let max_width = grid.iter().map(|row| row.len()).max().unwrap();
            grid.extend((0..extend_height).map(|_| Vec::new()));
            for (row_index, row) in grid.iter_mut().enumerate() {
                let mut col = row.len();
                row.resize_with(max_width, || {
                    let result = (self.transform)(Point::new(row_index, col), extend_value);
                    col += 1;
                    result
                })
            }
        }

        let max_width = grid.iter().map(|row| row.len()).max().unwrap();
        let height = grid.len();
        Grid(
            Array::from_iter(grid.into_iter().flat_map(|row| row.into_iter()))
                .into_shape((height, max_width))
                .unwrap(),
        )
    }
}
