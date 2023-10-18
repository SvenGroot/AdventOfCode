use std::fmt::Display;

use ndarray::{iter::Lanes, prelude::*};

use super::{Grid, Rectangle};

pub struct SubGrid<'a, T>(ArrayView2<'a, T>);

impl<'a, T> SubGrid<'a, T> {
    pub fn new(grid: &'a Grid<T>, bounds: Rectangle) -> Self {
        Self(grid.0.slice(s![
            bounds.top_left().row()..=bounds.bottom_right().row(),
            bounds.top_left().col()..=bounds.bottom_right().col()
        ]))
    }

    pub fn rows(&self) -> Lanes<'_, T, Dim<[usize; 1]>> {
        self.0.rows()
    }
}

impl<'a, T: Display> Display for SubGrid<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for cell in row {
                write!(f, "{cell}")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
