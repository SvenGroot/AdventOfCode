use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use num::FromPrimitive;

use super::{Point3D, PointDiff3D};

#[derive(Debug, Clone, Copy)]
pub struct GenericPoint3D<T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy,
{
    x: T,
    y: T,
    z: T,
}

impl<T> GenericPoint3D<T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }
}

impl<T> TryFrom<Point3D> for GenericPoint3D<T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy + FromPrimitive,
{
    type Error = &'static str;

    fn try_from(value: Point3D) -> Result<Self, Self::Error> {
        Ok(Self {
            x: T::from_usize(value.x()).ok_or("Can't convert value.")?,
            y: T::from_usize(value.y()).ok_or("Can't convert value.")?,
            z: T::from_usize(value.z()).ok_or("Can't convert value.")?,
        })
    }
}

impl<T> TryFrom<PointDiff3D> for GenericPoint3D<T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy + FromPrimitive,
{
    type Error = &'static str;

    fn try_from(value: PointDiff3D) -> Result<Self, Self::Error> {
        Ok(Self {
            x: T::from_isize(value.x()).ok_or("Can't convert value.")?,
            y: T::from_isize(value.y()).ok_or("Can't convert value.")?,
            z: T::from_isize(value.z()).ok_or("Can't convert value.")?,
        })
    }
}

impl<T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy> Add
    for GenericPoint3D<T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.y + rhs.z,
        }
    }
}

impl<T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy> AddAssign
    for GenericPoint3D<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy> Sub
    for GenericPoint3D<T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.y - rhs.z,
        }
    }
}

impl<T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy> SubAssign
    for GenericPoint3D<T>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy + FromStr> FromStr
    for GenericPoint3D<T>
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, rest) = s.split_once(',').unwrap();
        let (y, z) = rest.split_once(',').unwrap();
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

impl<T: Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Copy + Display> Display
    for GenericPoint3D<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
