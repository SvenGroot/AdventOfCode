use std::{
    fmt::Display,
    num::TryFromIntError,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use super::{Matrix3D, ParsePointError, Point3D};

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

    pub fn x_mut(&mut self) -> &mut isize {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut isize {
        &mut self.y
    }

    pub fn z_mut(&mut self) -> &mut isize {
        &mut self.z
    }

    pub fn into_point(self) -> Option<Point3D> {
        self.try_into().ok()
    }

    pub fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum(), self.z.signum())
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn rotate_x(&self) -> Self {
        Matrix3D::ROTATE_X * *self
    }

    pub fn rotate_y(&self) -> Self {
        Matrix3D::ROTATE_Y * *self
    }

    pub fn rotate_z(&self) -> Self {
        Matrix3D::ROTATE_Z * *self
    }

    pub fn manhattan_distance(&self) -> usize {
        let abs = self.abs();
        abs.x as usize + abs.y as usize + abs.z as usize
    }
}

impl TryFrom<Point3D> for PointDiff3D {
    type Error = TryFromIntError;

    fn try_from(value: Point3D) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x().try_into()?,
            y: value.y().try_into()?,
            z: value.z().try_into()?,
        })
    }
}

// Parses from "x,y,z" format
impl FromStr for PointDiff3D {
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

impl From<(isize, isize, isize)> for PointDiff3D {
    fn from(value: (isize, isize, isize)) -> Self {
        PointDiff3D::new(value.0, value.1, value.2)
    }
}

impl Display for PointDiff3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
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

impl Neg for PointDiff3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
