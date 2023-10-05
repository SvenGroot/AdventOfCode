use std::fmt::Display;

use super::{Grid, Rectangle};

pub struct SubGrid<'a, T>(&'a Grid<T>, Rectangle);

impl<'a, T> SubGrid<'a, T> {
    pub fn new(grid: &'a Grid<T>, bounds: Rectangle) -> Self {
        Self(grid, bounds)
    }

    pub fn values(&self) -> impl Iterator<Item = &'a T> + '_ {
        self.1.points().map(|pt| &self.0[pt])
    }
}

impl<'a, T: Display> Display for SubGrid<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_row = None;
        for pos in self.1.points() {
            if let Some(last_row_val) = last_row {
                if last_row_val != pos.row() {
                    writeln!(f)?;
                    last_row = Some(pos.row());
                }
            } else {
                last_row = Some(pos.row())
            }

            write!(f, "{}", self.0[pos])?;
        }

        Ok(())
    }
}
