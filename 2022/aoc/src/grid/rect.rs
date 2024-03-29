use std::num::NonZeroUsize;

use super::Point;

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
