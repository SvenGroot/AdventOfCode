use std::{
    fmt::Display,
    num::{ParseIntError, TryFromIntError},
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use thiserror::Error;

use super::PointDiff3D;

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

    pub fn line_to(&self, other: Point3D) -> Option<LineIterator3D> {
        // One of them must match, can't handle non-straight lines yet.
        if self.x != other.x && self.y != other.y && self.z != other.z {
            return None;
        }

        let direction = other.diff(*self)?.signum();
        Some(LineIterator3D {
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
            x: value.x().try_into()?,
            y: value.y().try_into()?,
            z: value.z().try_into()?,
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

// Parses from "x,y,z" format
impl FromStr for Point3D {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParsePointError::MissingDelimiter)?;
        let (y, z) = y.split_once(',').ok_or(ParsePointError::MissingDelimiter)?;
        Ok(Self {
            x: x.trim().parse()?,
            y: y.trim().parse()?,
            z: z.trim().parse()?,
        })
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x(), self.y(), self.z())
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

pub struct LineIterator3D {
    current: Point3D,
    end: Point3D,
    direction: PointDiff3D,
    done: bool,
}

impl Iterator for LineIterator3D {
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
