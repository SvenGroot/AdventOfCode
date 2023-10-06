use std::{
    convert::TryFrom,
    num::{NonZeroUsize, ParseIntError, TryFromIntError},
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
    str::FromStr,
};
use thiserror::Error;

#[derive(Clone)]
pub struct Grid3D<T: Clone>(Vec<Vec<Vec<T>>>);

impl<T: Clone> Grid3D<T> {
    pub fn new(height: NonZeroUsize, width: NonZeroUsize, depth: NonZeroUsize, value: T) -> Self {
        Self(vec![
            vec![vec![value; width.into()]; height.into()];
            depth.into()
        ])
    }

    pub fn width(&self) -> usize {
        self.0[0][0].len()
    }

    pub fn height(&self) -> usize {
        self.0[0].len()
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: Point3D) -> Option<&T> {
        self.0.get(index.x)?.get(index.y)?.get(index.z)
    }

    pub fn get_mut(&mut self, index: Point3D) -> Option<&mut T> {
        self.0.get_mut(index.x)?.get_mut(index.y)?.get_mut(index.z)
    }

    pub fn map<U: Clone>(&self, mut f: impl FnMut(&T) -> U) -> Grid3D<U> {
        let grid = self
            .0
            .iter()
            .map(|x| x.iter().map(|y| y.iter().map(&mut f).collect()).collect())
            .collect();

        Grid3D(grid)
    }

    pub fn cells(&self) -> impl Iterator<Item = (Point3D, &T)> {
        self.0.iter().enumerate().flat_map(|(x, y_vec)| {
            y_vec.iter().enumerate().flat_map(move |(y, z_vec)| {
                z_vec
                    .iter()
                    .enumerate()
                    .map(move |(z, cell)| (Point3D::new(x, y, z), cell))
            })
        })
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (Point3D, &mut T)> {
        self.0.iter_mut().enumerate().flat_map(|(x, y_vec)| {
            y_vec.iter_mut().enumerate().flat_map(move |(y, z_vec)| {
                z_vec
                    .iter_mut()
                    .enumerate()
                    .map(move |(z, cell)| (Point3D::new(x, y, z), cell))
            })
        })
    }

    pub fn straight_neighbors(&self, point: Point3D) -> impl Iterator<Item = Point3D> + '_ {
        point
            .straight_neighbors()
            .filter(|nb| nb.x < self.height() && nb.y < self.width() && nb.z < self.depth())
    }

    pub fn scan(&self, start: Point3D, direction: PointDiff3D) -> Scan3D<T> {
        Scan3D {
            grid: self,
            current: Some(start),
            direction,
        }
    }

    pub fn scan_mut(&mut self, start: Point3D, direction: PointDiff3D) -> ScanMut3D<T> {
        ScanMut3D {
            grid: self,
            current: Some(start),
            direction,
        }
    }
}

impl<T: Clone> Index<Point3D> for Grid3D<T> {
    type Output = T;

    fn index(&self, index: Point3D) -> &Self::Output {
        &self.0[index.x][index.y][index.z]
    }
}

impl<T: Clone> IndexMut<Point3D> for Grid3D<T> {
    fn index_mut(&mut self, index: Point3D) -> &mut Self::Output {
        &mut self.0[index.x][index.y][index.z]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug)]
pub struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Point3D {
    pub const fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn z(&self) -> usize {
        self.z
    }

    pub fn add(&self, x: isize, y: isize, z: isize) -> Option<Self> {
        self.add_diff(PointDiff3D::new(x, y, z))
    }

    pub fn add_diff(&self, diff: PointDiff3D) -> Option<Self> {
        // Can use checked_add_signed once stable.
        (self.into_diff()? + diff).into_point()
    }

    pub fn diff(&self, other: Point3D) -> Option<PointDiff3D> {
        Some(self.into_diff()? - other.into_diff()?)
    }

    pub fn into_diff(self) -> Option<PointDiff3D> {
        self.try_into().ok()
    }

    pub fn straight_neighbors(&self) -> Neighbors3D {
        Neighbors3D {
            point: *self,
            neighbors: &PointDiff3D::STRAIGHT_NEIGHBORS,
            index: 0,
        }
    }

    pub fn line_to(&self, other: Point3D) -> Option<Line> {
        // One of them must match, can't handle non-straight lines yet.
        if self.x != other.x && self.y != other.y {
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

impl Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.y + rhs.z,
        }
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.y - rhs.z,
        }
    }
}

impl SubAssign for Point3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl TryFrom<PointDiff3D> for Point3D {
    type Error = TryFromIntError;

    fn try_from(value: PointDiff3D) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
            z: value.z.try_into()?,
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

// Parses from "y,x" format
impl FromStr for Point3D {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParsePointError::MissingDelimiter)?;
        let (y, z) = y.split_once(',').ok_or(ParsePointError::MissingDelimiter)?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug)]
pub struct PointDiff3D {
    x: isize,
    y: isize,
    z: isize,
}

impl PointDiff3D {
    pub const UP: PointDiff3D = PointDiff3D::new(-1, 0, 0);
    pub const DOWN: PointDiff3D = PointDiff3D::new(1, 0, 0);
    pub const LEFT: PointDiff3D = PointDiff3D::new(0, -1, 0);
    pub const RIGHT: PointDiff3D = PointDiff3D::new(0, 1, 0);
    pub const IN: PointDiff3D = PointDiff3D::new(0, 0, 1);
    pub const OUT: PointDiff3D = PointDiff3D::new(0, 0, -1);

    pub const STRAIGHT_NEIGHBORS: [PointDiff3D; 6] = [
        Self::UP,
        Self::RIGHT,
        Self::DOWN,
        Self::LEFT,
        Self::IN,
        Self::OUT,
    ];

    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }

    pub fn z(&self) -> isize {
        self.z
    }

    pub fn into_point(self) -> Option<Point3D> {
        self.try_into().ok()
    }

    pub fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum(), self.z.signum())
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.signum())
    }
}

impl TryFrom<Point3D> for PointDiff3D {
    type Error = TryFromIntError;

    fn try_from(value: Point3D) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
            z: value.z.try_into()?,
        })
    }
}

impl Add for PointDiff3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for PointDiff3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for PointDiff3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for PointDiff3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

pub struct Neighbors3D {
    point: Point3D,
    neighbors: &'static [PointDiff3D],
    index: usize,
}

impl Iterator for Neighbors3D {
    type Item = Point3D;

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

pub struct Scan3D<'a, T: Clone> {
    grid: &'a Grid3D<T>,
    current: Option<Point3D>,
    direction: PointDiff3D,
}

impl<'a, T: Clone> Iterator for Scan3D<'a, T> {
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

pub struct ScanMut3D<'a, T: Clone> {
    grid: &'a mut Grid3D<T>,
    current: Option<Point3D>,
    direction: PointDiff3D,
}

impl<'a, T: Clone> Iterator for ScanMut3D<'a, T> {
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
    current: Point3D,
    end: Point3D,
    direction: PointDiff3D,
    done: bool,
}

impl Iterator for Line {
    type Item = Point3D;

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
        let mut neighbors = Point3D::new(2, 5, 9).straight_neighbors();
        assert_eq!(Some(Point3D::new(1, 5, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 6, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(3, 5, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 4, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 5, 10)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 5, 8)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = Point3D::new(0, 0, 0).straight_neighbors();
        assert_eq!(Some(Point3D::new(0, 1, 0)), neighbors.next());
        assert_eq!(Some(Point3D::new(1, 0, 0)), neighbors.next());
        assert_eq!(Some(Point3D::new(0, 0, 1)), neighbors.next());
        assert_eq!(None, neighbors.next());
    }

    #[test]
    fn test_grid_straight_neighbors() {
        let grid = Grid3D::<u8>::new(
            10.try_into().unwrap(),
            20.try_into().unwrap(),
            30.try_into().unwrap(),
            0,
        );

        let mut neighbors = grid.straight_neighbors(Point3D::new(2, 5, 9));
        assert_eq!(Some(Point3D::new(1, 5, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 6, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(3, 5, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 4, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 5, 10)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 5, 8)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.straight_neighbors(Point3D::new(0, 0, 0));
        assert_eq!(Some(Point3D::new(0, 1, 0)), neighbors.next());
        assert_eq!(Some(Point3D::new(1, 0, 0)), neighbors.next());
        assert_eq!(Some(Point3D::new(0, 0, 1)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.straight_neighbors(Point3D::new(9, 19, 29));
        assert_eq!(Some(Point3D::new(8, 19, 29)), neighbors.next());
        assert_eq!(Some(Point3D::new(9, 18, 29)), neighbors.next());
        assert_eq!(Some(Point3D::new(9, 19, 28)), neighbors.next());
        assert_eq!(None, neighbors.next());
    }
}
