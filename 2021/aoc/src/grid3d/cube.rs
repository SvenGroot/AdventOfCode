use std::num::NonZeroUsize;

use super::PointDiff3D;

/// A rectangle of PointDiff3D values (which are signed).
#[derive(Debug, Copy, Clone)]
pub struct DiffCube {
    top_left_front: PointDiff3D,
    bottom_right_back: PointDiff3D,
}

impl DiffCube {
    pub fn new(top_left_front: PointDiff3D, bottom_right_back: PointDiff3D) -> Self {
        assert!(
            bottom_right_back.x() >= top_left_front.x()
                && bottom_right_back.y() >= top_left_front.y()
                && bottom_right_back.z() >= top_left_front.z()
        );

        Self {
            top_left_front,
            bottom_right_back,
        }
    }

    pub fn from_size(
        top_left_front: PointDiff3D,
        width: NonZeroUsize,
        height: NonZeroUsize,
        depth: NonZeroUsize,
    ) -> Self {
        Self {
            top_left_front,
            bottom_right_back: PointDiff3D::new(
                top_left_front.x() + height.get() as isize - 1,
                top_left_front.y() + width.get() as isize - 1,
                top_left_front.z() + depth.get() as isize - 1,
            ),
        }
    }

    pub fn from_points<'a>(iter: impl Iterator<Item = &'a PointDiff3D>) -> Self {
        let (top_left_front, bottom_right_back) = iter.fold(
            (
                PointDiff3D::new(isize::MAX, isize::MAX, isize::MAX),
                PointDiff3D::new(isize::MIN, isize::MIN, isize::MAX),
            ),
            |(top_left, bottom_right), new| {
                (
                    PointDiff3D::new(
                        top_left.x().min(new.x()),
                        top_left.y().min(new.y()),
                        top_left.z().min(new.z()),
                    ),
                    PointDiff3D::new(
                        bottom_right.x().max(new.x()),
                        bottom_right.y().max(new.y()),
                        top_left.z().max(new.z()),
                    ),
                )
            },
        );

        Self::new(top_left_front, bottom_right_back)
    }

    pub fn top_left_front(&self) -> PointDiff3D {
        self.top_left_front
    }

    pub fn bottom_right_back(&self) -> PointDiff3D {
        self.bottom_right_back
    }

    pub fn width(&self) -> usize {
        (self.bottom_right_back.y() - self.top_left_front.y() + 1) as usize
    }

    pub fn height(&self) -> usize {
        (self.bottom_right_back.x() - self.top_left_front.x() + 1) as usize
    }

    pub fn depth(&self) -> usize {
        (self.bottom_right_back.z() - self.top_left_front.z() + 1) as usize
    }

    pub fn contains(&self, point: PointDiff3D) -> bool {
        self.top_left_front.x() <= point.x()
            && self.top_left_front.y() <= point.y()
            && self.top_left_front.z() <= point.z()
            && point.x() <= self.bottom_right_back.x()
            && point.y() <= self.bottom_right_back.y()
            && point.z() <= self.bottom_right_back.z()
    }

    /// Wholly contains the other cube.
    pub fn contains_cube(&self, other: &DiffCube) -> bool {
        self.contains(other.top_left_front()) && self.contains(other.bottom_right_back())
    }

    pub fn points(&self) -> impl Iterator<Item = PointDiff3D> + '_ {
        (self.top_left_front.x()..=self.bottom_right_back.x()).flat_map(move |x| {
            (self.top_left_front.y()..=self.bottom_right_back.y()).flat_map(move |y| {
                (self.top_left_front.z()..=self.bottom_right_back().z())
                    .map(move |z| PointDiff3D::new(x, y, z))
            })
        })
    }
}
