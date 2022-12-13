use std::{
    convert::TryFrom,
    fmt::Display,
    num::{NonZeroUsize, TryFromIntError},
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
    path::Path,
};

use crate::get_input;

#[derive(Clone)]
pub struct Grid<T: Clone>(Vec<Vec<T>>);

pub fn grid_from_file(path: impl AsRef<Path>) -> Grid<u8> {
    // TODO: Check grid is rectangular
    let grid = get_input(path)
        .map(|line| line.as_bytes().to_vec())
        .collect();

    Grid(grid)
}

impl<T: Clone> Grid<T> {
    pub fn new(height: NonZeroUsize, width: NonZeroUsize, value: T) -> Self {
        Self(vec![vec![value; width.into()]; height.into()])
    }

    pub fn get(&self, index: Point) -> Option<&T> {
        self.0.get(index.row)?.get(index.col)
    }

    pub fn get_mut(&mut self, index: Point) -> Option<&mut T> {
        self.0.get_mut(index.row)?.get_mut(index.col)
    }

    pub fn map<U: Clone>(&self, mut f: impl FnMut(&T) -> U) -> Grid<U> {
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

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (Point, &mut T)> {
        self.0.iter_mut().enumerate().flat_map(|(row, row_vec)| {
            row_vec
                .iter_mut()
                .enumerate()
                .map(move |(col, cell)| (Point::new(row, col), cell))
        })
    }

    pub fn straight_neighbors(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        point
            .straight_neighbors()
            .filter(|nb| nb.row < self.height() && nb.col < self.width())
    }
}

impl<T: Clone> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.row][index.col]
    }
}

impl<T: Clone> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.row][index.col]
    }
}

impl<T: Clone + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug)]
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
        self.add_diff(PointDiff::new(row, col))
    }

    pub fn add_diff(&self, diff: PointDiff) -> Option<Self> {
        // Can use checked_add_signed once stable.
        (self.into_diff()? + diff).into_point()
    }

    pub fn diff(&self, other: Point) -> Option<PointDiff> {
        Some(self.into_diff()? - other.into_diff()?)
    }

    pub fn into_diff(self) -> Option<PointDiff> {
        self.try_into().ok()
    }

    pub fn straight_neighbors(&self) -> Neighbors {
        Neighbors {
            point: *self,
            neighbors: &PointDiff::STRAIGHT_NEIGHBORS,
            index: 0,
        }
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

impl TryFrom<PointDiff> for Point {
    type Error = TryFromIntError;

    fn try_from(value: PointDiff) -> Result<Self, Self::Error> {
        Ok(Self {
            row: value.row.try_into()?,
            col: value.col.try_into()?,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug)]
pub struct PointDiff {
    row: isize,
    col: isize,
}

impl PointDiff {
    pub const UP: PointDiff = PointDiff::new(-1, 0);
    pub const DOWN: PointDiff = PointDiff::new(1, 0);
    pub const LEFT: PointDiff = PointDiff::new(0, -1);
    pub const RIGHT: PointDiff = PointDiff::new(0, 1);

    pub const STRAIGHT_NEIGHBORS: [PointDiff; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];

    pub const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn row(&self) -> isize {
        self.row
    }

    pub fn col(&self) -> isize {
        self.col
    }

    pub fn into_point(self) -> Option<Point> {
        self.try_into().ok()
    }
}

impl TryFrom<Point> for PointDiff {
    type Error = TryFromIntError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        Ok(Self {
            row: value.row.try_into()?,
            col: value.col.try_into()?,
        })
    }
}

impl Add for PointDiff {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl AddAssign for PointDiff {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl Sub for PointDiff {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl SubAssign for PointDiff {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}

pub struct Neighbors {
    point: Point,
    neighbors: &'static [PointDiff],
    index: usize,
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = None;
        while item.is_none() && self.index < self.neighbors.len() {
            let diff = self.neighbors[self.index];
            self.index += 1;
            item = self.point.add_diff(diff);
        }

        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_straight_neighbors() {
        let mut neighbors = Point::new(2, 5).straight_neighbors();
        assert_eq!(Some(Point::new(1, 5)), neighbors.next());
        assert_eq!(Some(Point::new(2, 6)), neighbors.next());
        assert_eq!(Some(Point::new(3, 5)), neighbors.next());
        assert_eq!(Some(Point::new(2, 4)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = Point::new(0, 0).straight_neighbors();
        assert_eq!(Some(Point::new(0, 1)), neighbors.next());
        assert_eq!(Some(Point::new(1, 0)), neighbors.next());
        assert_eq!(None, neighbors.next());
    }

    #[test]
    fn test_grid_straight_neighbors() {
        let grid = Grid::<u8>::new(10.try_into().unwrap(), 20.try_into().unwrap(), 0);
        let mut neighbors = grid.straight_neighbors(Point::new(2, 5));
        assert_eq!(Some(Point::new(1, 5)), neighbors.next());
        assert_eq!(Some(Point::new(2, 6)), neighbors.next());
        assert_eq!(Some(Point::new(3, 5)), neighbors.next());
        assert_eq!(Some(Point::new(2, 4)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.straight_neighbors(Point::new(0, 0));
        assert_eq!(Some(Point::new(0, 1)), neighbors.next());
        assert_eq!(Some(Point::new(1, 0)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.straight_neighbors(Point::new(9, 19));
        assert_eq!(Some(Point::new(8, 19)), neighbors.next());
        assert_eq!(Some(Point::new(9, 18)), neighbors.next());
        assert_eq!(None, neighbors.next());
    }
}
