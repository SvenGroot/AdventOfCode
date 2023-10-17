use std::{
    num::TryFromIntError,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::Point3D;

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
            x: value.x().try_into()?,
            y: value.y().try_into()?,
            z: value.z().try_into()?,
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
