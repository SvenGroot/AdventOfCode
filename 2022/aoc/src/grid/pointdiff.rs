use std::{
    num::TryFromIntError,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::Point;

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
