mod builder;
mod subgrid;

use std::{
    convert::TryFrom,
    fmt::Display,
    num::{NonZeroUsize, ParseIntError, TryFromIntError},
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

pub use builder::GridBuilder;
pub use subgrid::SubGrid;

pub struct Grid<T>(Vec<Vec<T>>);

pub fn grid_from_file(path: impl AsRef<Path>) -> Grid<u8> {
    GridBuilder::from_file(path).build()
}

impl<T: Clone> Grid<T> {
    pub fn new(height: NonZeroUsize, width: NonZeroUsize, value: T) -> Self {
        Self(vec![vec![value; width.into()]; height.into()])
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

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
            .map(|row| row.iter().map(&mut f).collect())
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

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (Point, &mut T)> {
        self.0.iter_mut().enumerate().flat_map(|(row, row_vec)| {
            row_vec
                .iter_mut()
                .enumerate()
                .map(move |(col, cell)| (Point::new(row, col), cell))
        })
    }

    pub fn edge_cells(&self) -> impl Iterator<Item = (Point, &T)> {
        // This could obviously be done much more efficiently.
        let last_row = self.height() - 1;
        let last_col = self.width() - 1;
        self.cells().filter(move |(point, _)| {
            point.row == 0 || point.row == last_row || point.col == 0 || point.col == last_col
        })
    }

    pub fn edge_cells_mut(&mut self) -> impl Iterator<Item = (Point, &mut T)> {
        let last_row = self.height() - 1;
        let last_col = self.width() - 1;
        self.cells_mut().filter(move |(point, _)| {
            point.row == 0 || point.row == last_row || point.col == 0 || point.col == last_col
        })
    }

    pub fn straight_neighbors(&self, point: Point) -> impl Iterator<Item = Point> {
        self.neighbors(point, &PointDiff::STRAIGHT_NEIGHBORS)
    }

    pub fn all_neighbors(&self, point: Point) -> impl Iterator<Item = Point> {
        self.neighbors(point, &PointDiff::ALL_NEIGHBORS)
    }

    pub fn neighbors<'a>(
        &self,
        point: Point,
        neighbors: &'a [PointDiff],
    ) -> impl Iterator<Item = Point> + 'a {
        let height = self.height();
        let width = self.width();
        point
            .neighbors(neighbors)
            .filter(move |nb| nb.row < height && nb.col < width)
    }

    // Adds a diff to a point, only if the result is on the grid.
    pub fn add_point(&self, point: Point, diff: PointDiff) -> Option<Point> {
        let result = point.add_diff(diff)?;
        (result.row < self.height() && result.col < self.width()).then_some(result)
    }

    pub fn find_in_row(&self, row: usize, predicate: impl FnMut(&T) -> bool) -> Option<Point> {
        let col = self
            .scan(Point::new(row, 0), PointDiff::RIGHT)
            .position(predicate)?;

        Some(Point::new(row, col))
    }

    pub fn rfind_in_row(&self, row: usize, predicate: impl FnMut(&T) -> bool) -> Option<Point> {
        let col = self
            .scan(Point::new(row, self.width() - 1), PointDiff::LEFT)
            .position(predicate)?;

        Some(Point::new(row, self.width() - col - 1))
    }

    pub fn find_in_col(&self, col: usize, predicate: impl FnMut(&T) -> bool) -> Option<Point> {
        let row = self
            .scan(Point::new(0, col), PointDiff::DOWN)
            .position(predicate)?;

        Some(Point::new(row, col))
    }

    pub fn rfind_in_col(&self, col: usize, predicate: impl FnMut(&T) -> bool) -> Option<Point> {
        let row = self
            .scan(Point::new(self.height() - 1, col), PointDiff::UP)
            .position(predicate)?;

        Some(Point::new(self.height() - row - 1, col))
    }

    pub fn scan(&self, start: Point, direction: PointDiff) -> Scan<T> {
        Scan {
            grid: self,
            current: Some(start),
            direction,
        }
    }

    pub fn scan_mut(&mut self, start: Point, direction: PointDiff) -> ScanMut<T> {
        ScanMut {
            grid: self,
            current: Some(start),
            direction,
        }
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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

impl<T: Display> Display for Grid<T> {
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
            row: self.row.checked_add_signed(diff.row)?,
            col: self.col.checked_add_signed(diff.col)?,
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
        diff.row.abs() <= 1 && diff.col.abs() <= 1
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
        // One of them must match, can't handle non-straight lines yet.
        if self.row != other.row && self.col != other.col {
            return None;
        }

        let direction = other.diff(*self)?.signum();
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
            row: value.row.try_into()?,
            col: value.col.try_into()?,
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

#[derive(Error, Debug)]
pub enum ParsePointError {
    #[error("doesn't have a delimiter")]
    MissingDelimiter,
    #[error("error parsing int")]
    ParseIntError(#[from] ParseIntError),
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
    pub const UP_LEFT: PointDiff = PointDiff::new(-1, -1);
    pub const UP_RIGHT: PointDiff = PointDiff::new(-1, 1);
    pub const DOWN_LEFT: PointDiff = PointDiff::new(1, -1);
    pub const DOWN_RIGHT: PointDiff = PointDiff::new(1, 1);

    pub const STRAIGHT_NEIGHBORS: [PointDiff; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];
    pub const ALL_NEIGHBORS: [PointDiff; 8] = [
        Self::UP,
        Self::RIGHT,
        Self::DOWN,
        Self::LEFT,
        Self::UP_LEFT,
        Self::UP_RIGHT,
        Self::DOWN_LEFT,
        Self::DOWN_RIGHT,
    ];

    pub const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn from_char(input: u8, up: u8, right: u8, down: u8, left: u8) -> Option<Self> {
        if input == up {
            Some(PointDiff::UP)
        } else if input == right {
            Some(PointDiff::RIGHT)
        } else if input == down {
            Some(PointDiff::DOWN)
        } else if input == left {
            Some(PointDiff::LEFT)
        } else {
            None
        }
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

    pub fn signum(&self) -> Self {
        Self::new(self.row.signum(), self.col.signum())
    }

    pub fn abs(&self) -> Self {
        Self::new(self.row.abs(), self.col.abs())
    }

    pub fn get_dir_char(&self) -> Option<char> {
        Some(match *self {
            PointDiff::UP => '^',
            PointDiff::DOWN => 'v',
            PointDiff::LEFT => '<',
            PointDiff::RIGHT => '>',
            _ => return None,
        })
    }

    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Right => Self::new(self.col, -self.row),
            Rotation::Left => Self::new(-self.col, self.row),
        }
    }

    pub fn invert(&self) -> Self {
        Self::new(-self.row, -self.col)
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

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        assert!(bottom_right.row >= top_left.row && bottom_right.col >= top_left.col);
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn from_size(top_left: Point, width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self {
            top_left,
            bottom_right: Point::new(
                top_left.row + height.get() - 1,
                top_left.col + width.get() - 1,
            ),
        }
    }

    pub fn top_left(&self) -> Point {
        self.top_left
    }

    pub fn bottom_right(&self) -> Point {
        self.bottom_right
    }

    pub fn width(&self) -> usize {
        self.bottom_right.col - self.top_left.col + 1
    }

    pub fn height(&self) -> usize {
        self.bottom_right.row - self.top_left.row + 1
    }

    pub fn contains(&self, point: Point) -> bool {
        self.top_left.row <= point.row
            && self.top_left.col <= point.col
            && point.row <= self.bottom_right.row
            && point.col <= self.bottom_right.col
    }

    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.top_left.row..=self.bottom_right.row).flat_map(|row| {
            (self.top_left.col..=self.bottom_right.col).map(move |col| Point::new(row, col))
        })
    }
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

pub struct Scan<'a, T> {
    grid: &'a Grid<T>,
    current: Option<Point>,
    direction: PointDiff,
}

impl<'a, T> Iterator for Scan<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.current?)?;
        self.current = if let Some(p) = self.current {
            p.add_diff(self.direction)
        } else {
            None
        };

        Some(result)
    }
}

pub struct ScanMut<'a, T> {
    grid: &'a mut Grid<T>,
    current: Option<Point>,
    direction: PointDiff,
}

impl<'a, T> Iterator for ScanMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get_mut(self.current?)?;
        self.current = if let Some(p) = self.current {
            p.add_diff(self.direction)
        } else {
            None
        };

        // Work around borrow checker limitation.
        unsafe { Some(&mut *(result as *mut T)) }
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

    #[test]
    fn test_non_uniform() {
        let input = vec!["123".to_owned(), "12345".to_owned(), "12".to_owned()];
        let grid = GridBuilder::from_lines(input.into_iter())
            .extend(0, 0, b' ')
            .build();

        assert_eq!(5, grid.width());
        assert_eq!(3, grid.height());
        assert_eq!(&[b'1', b'2', b'3', b' ', b' '], grid.0[0].as_slice());
        assert_eq!(&[b'1', b'2', b'3', b'4', b'5'], grid.0[1].as_slice());
        assert_eq!(&[b'1', b'2', b' ', b' ', b' '], grid.0[2].as_slice());
    }

    #[test]
    fn test_rotate() {
        let dir = PointDiff::RIGHT;
        let dir = dir.rotate(Rotation::Right);
        assert_eq!(PointDiff::DOWN, dir);
        let dir = dir.rotate(Rotation::Right);
        assert_eq!(PointDiff::LEFT, dir);
        let dir = dir.rotate(Rotation::Right);
        assert_eq!(PointDiff::UP, dir);
        let dir = dir.rotate(Rotation::Right);
        assert_eq!(PointDiff::RIGHT, dir);

        let dir = dir.rotate(Rotation::Left);
        assert_eq!(PointDiff::UP, dir);
        let dir = dir.rotate(Rotation::Left);
        assert_eq!(PointDiff::LEFT, dir);
        let dir = dir.rotate(Rotation::Left);
        assert_eq!(PointDiff::DOWN, dir);
        let dir = dir.rotate(Rotation::Left);
        assert_eq!(PointDiff::RIGHT, dir);
    }
}
