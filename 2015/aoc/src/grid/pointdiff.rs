use std::{
    iter::Sum,
    num::TryFromIntError,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::Point;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug)]
pub struct PointDiff {
    row: isize,
    col: isize,
}

impl PointDiff {
    pub const ZERO: PointDiff = PointDiff::new(0, 0);
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

    /// Parses into one of four directions listed in `dirs`, in the order \[up, right, down, left].
    pub fn from_char(input: u8, dirs: [u8; 4]) -> Option<Self> {
        let index = dirs.iter().position(|dir| *dir == input)?;
        Some(Self::STRAIGHT_NEIGHBORS[index])
    }

    pub fn from_arrows(input: u8) -> Option<Self> {
        Self::from_char(input, [b'^', b'>', b'v', b'<'])
    }

    /// Parses into one of four directions listed in `dirs`, in the order \[up, right, down, left].
    pub fn from_str(input: impl AsRef<str>, dirs: [&str; 4]) -> Option<Self> {
        let input = input.as_ref();
        let index = dirs.iter().position(|dir| *dir == input)?;
        Some(Self::STRAIGHT_NEIGHBORS[index])
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
            row: value.row().try_into()?,
            col: value.col().try_into()?,
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

impl Mul<isize> for PointDiff {
    type Output = PointDiff;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

impl MulAssign<isize> for PointDiff {
    fn mul_assign(&mut self, rhs: isize) {
        self.row *= rhs;
        self.col *= rhs;
    }
}

impl SubAssign for PointDiff {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}

impl Sum for PointDiff {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(PointDiff::default(), |x, y| x + y)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

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
