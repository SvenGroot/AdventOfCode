use std::{
    num::{ParseIntError, TryFromIntError},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use thiserror::Error;

use super::PointDiff;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub const fn new(row: usize, col: usize) -> Self {
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
        Some(Self {
            row: self.row.checked_add_signed(diff.row())?,
            col: self.col.checked_add_signed(diff.col())?,
        })
    }

    pub fn diff(&self, other: Point) -> Option<PointDiff> {
        Some(self.into_diff()? - other.into_diff()?)
    }

    pub fn into_diff(self) -> Option<PointDiff> {
        self.try_into().ok()
    }

    pub fn is_adjacent(&self, other: Point) -> bool {
        let diff = self.diff(other).unwrap();
        diff.row().abs() <= 1 && diff.col().abs() <= 1
    }

    pub fn straight_neighbors(&self) -> Neighbors<'static> {
        Neighbors {
            point: *self,
            neighbors: &PointDiff::STRAIGHT_NEIGHBORS,
            index: 0,
        }
    }

    pub fn all_neighbors(&self) -> Neighbors<'static> {
        Neighbors {
            point: *self,
            neighbors: &PointDiff::ALL_NEIGHBORS,
            index: 0,
        }
    }

    pub fn neighbors<'a>(&self, neighbors: &'a [PointDiff]) -> Neighbors<'a> {
        Neighbors {
            point: *self,
            neighbors,
            index: 0,
        }
    }

    pub fn line_to(&self, other: Point) -> Option<Line> {
        let diff = other.diff(*self)?;
        // It must be either a straight or diagonal line.
        if self.row != other.row && self.col != other.col && diff.row().abs() != diff.col().abs() {
            return None;
        }

        let direction = diff.signum();
        Some(Line {
            current: *self,
            end: other,
            direction,
            done: false,
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

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

impl MulAssign<usize> for Point {
    fn mul_assign(&mut self, rhs: usize) {
        self.row *= rhs;
        self.col *= rhs;
    }
}

impl Add<PointDiff> for Point {
    type Output = Point;

    fn add(self, rhs: PointDiff) -> Self::Output {
        self.add_diff(rhs).unwrap()
    }
}

impl AddAssign<PointDiff> for Point {
    fn add_assign(&mut self, rhs: PointDiff) {
        *self = self.add_diff(rhs).unwrap();
    }
}

impl TryFrom<PointDiff> for Point {
    type Error = TryFromIntError;

    fn try_from(value: PointDiff) -> Result<Self, Self::Error> {
        Ok(Self {
            row: value.row().try_into()?,
            col: value.col().try_into()?,
        })
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0,
            col: value.1,
        }
    }
}

// Parses from "col,row" format
impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (col, row) = s.split_once(',').ok_or(ParsePointError::MissingDelimiter)?;
        Ok(Self {
            row: row.parse()?,
            col: col.parse()?,
        })
    }
}

#[derive(Error, Debug)]
pub enum ParsePointError {
    #[error("doesn't have a delimiter")]
    MissingDelimiter,
    #[error("error parsing int")]
    ParseIntError(#[from] ParseIntError),
}

pub struct Neighbors<'a> {
    point: Point,
    neighbors: &'a [PointDiff],
    index: usize,
}

impl<'a> Iterator for Neighbors<'a> {
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

pub struct Line {
    current: Point,
    end: Point,
    direction: PointDiff,
    done: bool,
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        (!self.done).then(|| {
            let result = self.current;
            self.done = self.current == self.end;
            if !self.done {
                self.current = self.current.add_diff(self.direction).unwrap()
            }

            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_neighbors() {
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
}
