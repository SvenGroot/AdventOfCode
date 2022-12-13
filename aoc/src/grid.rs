use std::{
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
    path::Path,
};

use crate::get_input;

pub struct Grid<T>(Vec<Vec<T>>);

pub fn grid_from_file(path: impl AsRef<Path>) -> Grid<u8> {
    let grid = get_input(path)
        .map(|line| line.as_bytes().to_vec())
        .collect();

    Grid(grid)
}

impl<T> Grid<T> {
    pub fn get(&self, index: Point) -> Option<&T> {
        self.0.get(index.row)?.get(index.col)
    }

    pub fn get_mut(&mut self, index: Point) -> Option<&mut T> {
        self.0.get_mut(index.row)?.get_mut(index.col)
    }

    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> Grid<U> {
        let grid = self
            .0
            .iter()
            .map(|row| row.iter().map(|cell| f(cell)).collect())
            .collect();

        Grid(grid)
    }

    pub fn cells(&self) -> impl Iterator<Item = (Point, &T)> {
        self.0.iter().enumerate().flat_map(|(row, row_vec)| {
            row_vec
                .iter()
                .enumerate()
                .map(move |(col, cell)| (Point::new(row, col), cell))
        })
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.row][index.col]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.row][index.col]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn add(&self, row: isize, col: isize) -> Option<Self> {
        // Can use checked_add_signed once stable.
        let row = isize::try_from(self.row).ok()? + row;
        let col = isize::try_from(self.col).ok()? + col;
        Some(Self {
            row: row.try_into().ok()?,
            col: col.try_into().ok()?,
        })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}
