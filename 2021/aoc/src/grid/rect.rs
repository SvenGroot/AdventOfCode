use std::{borrow::Borrow, num::NonZeroUsize};

use super::{Point, PointDiff};

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        assert!(bottom_right.row() >= top_left.row() && bottom_right.col() >= top_left.col());
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn from_size(top_left: Point, width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self {
            top_left,
            bottom_right: Point::new(
                top_left.row() + height.get() - 1,
                top_left.col() + width.get() - 1,
            ),
        }
    }

    pub fn from_points(iter: impl Iterator<Item = impl Borrow<Point>>) -> Self {
        let (top_left, bottom_right) = iter.fold(
            (Point::new(usize::MAX, usize::MAX), Point::default()),
            |(top_left, bottom_right), new| {
                let new = new.borrow();
                (
                    Point::new(top_left.row().min(new.row()), top_left.col().min(new.col())),
                    Point::new(
                        bottom_right.row().max(new.row()),
                        bottom_right.col().max(new.col()),
                    ),
                )
            },
        );

        Rectangle::new(top_left, bottom_right)
    }

    pub fn top_left(&self) -> Point {
        self.top_left
    }

    pub fn bottom_right(&self) -> Point {
        self.bottom_right
    }

    pub fn width(&self) -> usize {
        self.bottom_right.col() - self.top_left.col() + 1
    }

    pub fn height(&self) -> usize {
        self.bottom_right.row() - self.top_left.row() + 1
    }

    pub fn contains(&self, point: Point) -> bool {
        self.top_left.row() <= point.row()
            && self.top_left.col() <= point.col()
            && point.row() <= self.bottom_right.row()
            && point.col() <= self.bottom_right.col()
    }

    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.top_left.row()..=self.bottom_right.row()).flat_map(|row| {
            (self.top_left.col()..=self.bottom_right.col()).map(move |col| Point::new(row, col))
        })
    }
}

/// A rectangle of PointDiff values (which are signed).
#[derive(Debug, Copy, Clone)]
pub struct DiffRectangle {
    top_left: PointDiff,
    bottom_right: PointDiff,
}

impl DiffRectangle {
    pub fn new(top_left: PointDiff, bottom_right: PointDiff) -> Self {
        assert!(bottom_right.row() >= top_left.row() && bottom_right.col() >= top_left.col());
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn from_size(top_left: PointDiff, width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self {
            top_left,
            bottom_right: PointDiff::new(
                top_left.row() + height.get() as isize - 1,
                top_left.col() + width.get() as isize - 1,
            ),
        }
    }

    pub fn from_points<'a>(iter: impl Iterator<Item = &'a PointDiff>) -> Self {
        let (top_left, bottom_right) = iter.fold(
            (
                PointDiff::new(isize::MAX, isize::MAX),
                PointDiff::new(isize::MIN, isize::MIN),
            ),
            |(top_left, bottom_right), new| {
                (
                    PointDiff::new(top_left.row().min(new.row()), top_left.col().min(new.col())),
                    PointDiff::new(
                        bottom_right.row().max(new.row()),
                        bottom_right.col().max(new.col()),
                    ),
                )
            },
        );

        Self::new(top_left, bottom_right)
    }

    pub fn top_left(&self) -> PointDiff {
        self.top_left
    }

    pub fn bottom_right(&self) -> PointDiff {
        self.bottom_right
    }

    pub fn width(&self) -> usize {
        (self.bottom_right.col() - self.top_left.col() + 1) as usize
    }

    pub fn height(&self) -> usize {
        (self.bottom_right.row() - self.top_left.row() + 1) as usize
    }

    pub fn contains(&self, point: PointDiff) -> bool {
        self.top_left.row() <= point.row()
            && self.top_left.col() <= point.col()
            && point.row() <= self.bottom_right.row()
            && point.col() <= self.bottom_right.col()
    }

    pub fn points(&self) -> impl Iterator<Item = PointDiff> + '_ {
        (self.top_left.row()..=self.bottom_right.row()).flat_map(|row| {
            (self.top_left.col()..=self.bottom_right.col()).map(move |col| PointDiff::new(row, col))
        })
    }
}
