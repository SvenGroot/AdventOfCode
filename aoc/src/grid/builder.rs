use std::path::Path;

use crate::get_input;

use super::Grid;

pub struct GridBuilder<'path, T> {
    path: &'path Path,
    transform: Box<dyn Fn(u8) -> T>,
    extend: Option<(usize, usize, u8)>,
}

impl<'path> GridBuilder<'path, u8> {
    pub fn from_file(path: &'path impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref(),
            transform: Box::new(|byte| byte),
            extend: None,
        }
    }
}

impl<'path, T> GridBuilder<'path, T> {
    pub fn map<U>(self, transform: impl Fn(u8) -> U + Clone + 'static) -> GridBuilder<'path, U> {
        GridBuilder {
            path: self.path,
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

        grid.extend(get_input(self.path).map(|line| {
            let mut row: Vec<_> = (0..extend_width)
                .map(|_| (self.transform)(extend_value))
                .collect();

            row.extend(line.bytes().map(self.transform.as_ref()));
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
