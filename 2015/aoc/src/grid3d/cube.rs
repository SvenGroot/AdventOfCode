use std::num::NonZeroUsize;

use super::PointDiff3D;

/// A rectangle of PointDiff3D values (which are signed).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DiffCuboid {
    top_left_front: PointDiff3D,
    bottom_right_back: PointDiff3D,
}

impl DiffCuboid {
    pub fn new(top_left_front: PointDiff3D, bottom_right_back: PointDiff3D) -> Self {
        Self::try_new(top_left_front, bottom_right_back).unwrap()
    }

    pub fn try_new(top_left_front: PointDiff3D, bottom_right_back: PointDiff3D) -> Option<Self> {
        (bottom_right_back.x() >= top_left_front.x()
            && bottom_right_back.y() >= top_left_front.y()
            && bottom_right_back.z() >= top_left_front.z())
        .then_some(Self {
            top_left_front,
            bottom_right_back,
        })
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

    pub fn surface_area(&self) -> usize {
        2 * self.depth() * self.width()
            + 2 * self.width() * self.height()
            + 2 * self.height() * self.depth()
    }

    pub fn volume(&self) -> usize {
        self.width() * self.height() * self.depth()
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
    pub fn contains_cube(&self, other: &DiffCuboid) -> bool {
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

    pub fn split(&self, point: PointDiff3D, align: SplitAlign) -> Option<Vec<DiffCuboid>> {
        if !self.contains(point) {
            return None;
        }

        let x = [
            self.top_left_front.x(),
            if align.left { point.x() + 1 } else { point.x() },
            self.bottom_right_back.x() + 1,
        ];
        let y = [
            self.top_left_front.y(),
            if align.top { point.y() + 1 } else { point.y() },
            self.bottom_right_back.y() + 1,
        ];
        let z = [
            self.top_left_front.z(),
            if align.front {
                point.z() + 1
            } else {
                point.z()
            },
            self.bottom_right_back.z() + 1,
        ];

        let mut result = Vec::new();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let top_left_front = PointDiff3D::new(x[i], y[j], z[k]);
                    let bottom_right_back =
                        PointDiff3D::new(x[i + 1] - 1, y[j + 1] - 1, z[k + 1] - 1);

                    if let Some(cuboid) = Self::try_new(top_left_front, bottom_right_back) {
                        result.push(cuboid)
                    }
                }
            }
        }

        Some(result)
    }

    /// Returns a set of cubes that cover this cube, excluding the area that intersects with the
    /// other cube.
    pub fn diff(&self, other: &DiffCuboid) -> Vec<DiffCuboid> {
        // If we are completely contained in other, then this cube is just removed.
        if other.contains_cube(self) {
            // println!("Full overlap:");
            // for pt in self.points() {
            //     println!("{pt}");
            // }
            return Vec::new();
        }

        let x = [
            other.top_left_front().x().max(self.top_left_front().x()),
            other
                .bottom_right_back()
                .x()
                .min(self.bottom_right_back().x()),
        ];
        let y = [
            other.top_left_front().y().max(self.top_left_front().y()),
            other
                .bottom_right_back()
                .y()
                .min(self.bottom_right_back().y()),
        ];
        let z = [
            other.top_left_front().z().max(self.top_left_front().z()),
            other
                .bottom_right_back()
                .z()
                .min(self.bottom_right_back().z()),
        ];

        // No overlap if any of the points are the wrong way around.
        if x[0] > x[1] || y[0] > y[1] || z[0] > z[1] {
            return vec![*self];
        }

        let mut result = Vec::new();
        let mut cube_to_check = *self;
        for (i, x) in x.iter().enumerate() {
            for (j, y) in y.iter().enumerate() {
                for (k, z) in z.iter().enumerate() {
                    let align = SplitAlign::new(i != 0, j != 0, k != 0);
                    let point = (*x, *y, *z).into();
                    if let Some(cubes) = cube_to_check.split(point, align) {
                        for cube in cubes {
                            if cube.contains(point) {
                                cube_to_check = cube;
                            } else {
                                result.push(cube);
                            }
                        }
                    }
                }
            }
        }

        // The two cubes didn't overlap.
        if result.is_empty() {
            result.push(*self);
        } else {
            assert_eq!(
                self.volume(),
                result.iter().map(|c| c.volume()).sum::<usize>() + cube_to_check.volume()
            );

            assert!(other.contains_cube(&cube_to_check));
            // println!("Overlap:");
            // for pt in cube_to_check.points() {
            //     println!("{pt}");
            // }
        }

        result
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SplitAlign {
    left: bool,
    top: bool,
    front: bool,
}

impl SplitAlign {
    pub fn new(left: bool, top: bool, front: bool) -> Self {
        Self { left, top, front }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let cube = DiffCuboid::new((0, 0, 0).into(), (10, 10, 10).into());
        let cubes = cube.split((3, 4, 5).into(), SplitAlign::default()).unwrap();
        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((0, 0, 0).into(), (2, 3, 4).into()),
                DiffCuboid::new((0, 0, 5).into(), (2, 3, 10).into()),
                DiffCuboid::new((0, 4, 0).into(), (2, 10, 4).into()),
                DiffCuboid::new((0, 4, 5).into(), (2, 10, 10).into()),
                DiffCuboid::new((3, 0, 0).into(), (10, 3, 4).into()),
                DiffCuboid::new((3, 0, 5).into(), (10, 3, 10).into()),
                DiffCuboid::new((3, 4, 0).into(), (10, 10, 4).into()),
                DiffCuboid::new((3, 4, 5).into(), (10, 10, 10).into()),
            ]
        );

        let cube = DiffCuboid::new((0, 0, 0).into(), (10, 10, 10).into());
        let cubes = cube
            .split((3, 4, 5).into(), SplitAlign::new(true, false, false))
            .unwrap();

        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((0, 0, 0).into(), (3, 3, 4).into()),
                DiffCuboid::new((0, 0, 5).into(), (3, 3, 10).into()),
                DiffCuboid::new((0, 4, 0).into(), (3, 10, 4).into()),
                DiffCuboid::new((0, 4, 5).into(), (3, 10, 10).into()),
                DiffCuboid::new((4, 0, 0).into(), (10, 3, 4).into()),
                DiffCuboid::new((4, 0, 5).into(), (10, 3, 10).into()),
                DiffCuboid::new((4, 4, 0).into(), (10, 10, 4).into()),
                DiffCuboid::new((4, 4, 5).into(), (10, 10, 10).into()),
            ]
        );

        let cube = DiffCuboid::new((0, 0, 0).into(), (10, 10, 10).into());
        let cubes = cube
            .split((3, 4, 5).into(), SplitAlign::new(false, true, false))
            .unwrap();

        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((0, 0, 0).into(), (2, 4, 4).into()),
                DiffCuboid::new((0, 0, 5).into(), (2, 4, 10).into()),
                DiffCuboid::new((0, 5, 0).into(), (2, 10, 4).into()),
                DiffCuboid::new((0, 5, 5).into(), (2, 10, 10).into()),
                DiffCuboid::new((3, 0, 0).into(), (10, 4, 4).into()),
                DiffCuboid::new((3, 0, 5).into(), (10, 4, 10).into()),
                DiffCuboid::new((3, 5, 0).into(), (10, 10, 4).into()),
                DiffCuboid::new((3, 5, 5).into(), (10, 10, 10).into()),
            ]
        );

        let cube = DiffCuboid::new((0, 0, 0).into(), (10, 10, 10).into());
        let cubes = cube
            .split((3, 4, 5).into(), SplitAlign::new(false, false, true))
            .unwrap();

        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((0, 0, 0).into(), (2, 3, 5).into()),
                DiffCuboid::new((0, 0, 6).into(), (2, 3, 10).into()),
                DiffCuboid::new((0, 4, 0).into(), (2, 10, 5).into()),
                DiffCuboid::new((0, 4, 6).into(), (2, 10, 10).into()),
                DiffCuboid::new((3, 0, 0).into(), (10, 3, 5).into()),
                DiffCuboid::new((3, 0, 6).into(), (10, 3, 10).into()),
                DiffCuboid::new((3, 4, 0).into(), (10, 10, 5).into()),
                DiffCuboid::new((3, 4, 6).into(), (10, 10, 10).into()),
            ]
        );

        let cube = DiffCuboid::new((0, 0, 0).into(), (10, 10, 10).into());
        let cubes = cube.split((0, 4, 5).into(), SplitAlign::default()).unwrap();
        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((0, 0, 0).into(), (10, 3, 4).into()),
                DiffCuboid::new((0, 0, 5).into(), (10, 3, 10).into()),
                DiffCuboid::new((0, 4, 0).into(), (10, 10, 4).into()),
                DiffCuboid::new((0, 4, 5).into(), (10, 10, 10).into()),
            ]
        );

        let cube = DiffCuboid::new((0, 0, 0).into(), (10, 10, 10).into());
        let cubes = cube
            .split((0, 4, 5).into(), SplitAlign::new(true, false, false))
            .unwrap();

        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((0, 0, 0).into(), (0, 3, 4).into()),
                DiffCuboid::new((0, 0, 5).into(), (0, 3, 10).into()),
                DiffCuboid::new((0, 4, 0).into(), (0, 10, 4).into()),
                DiffCuboid::new((0, 4, 5).into(), (0, 10, 10).into()),
                DiffCuboid::new((1, 0, 0).into(), (10, 3, 4).into()),
                DiffCuboid::new((1, 0, 5).into(), (10, 3, 10).into()),
                DiffCuboid::new((1, 4, 0).into(), (10, 10, 4).into()),
                DiffCuboid::new((1, 4, 5).into(), (10, 10, 10).into()),
            ]
        );
    }

    #[test]
    fn test_diff() {
        let cube1 = DiffCuboid::new((0, 0, 0).into(), (10, 10, 10).into());
        let cube2 = DiffCuboid::new((3, 4, 5).into(), (15, 15, 15).into());
        let cubes = cube1.diff(&cube2);
        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((0, 0, 0).into(), (2, 3, 4).into()),
                DiffCuboid::new((0, 0, 5).into(), (2, 3, 10).into()),
                DiffCuboid::new((0, 4, 0).into(), (2, 10, 4).into()),
                DiffCuboid::new((0, 4, 5).into(), (2, 10, 10).into()),
                DiffCuboid::new((3, 0, 0).into(), (10, 3, 4).into()),
                DiffCuboid::new((3, 0, 5).into(), (10, 3, 10).into()),
                DiffCuboid::new((3, 4, 0).into(), (10, 10, 4).into()),
            ]
        );

        // Completely contained.
        let cube1 = DiffCuboid::new((0, 0, 0).into(), (10, 10, 10).into());
        let cube2 = DiffCuboid::new((1, 2, 3).into(), (6, 7, 8).into());
        let cubes = cube1.diff(&cube2);
        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((0, 0, 0).into(), (0, 1, 2).into()),
                DiffCuboid::new((0, 0, 3).into(), (0, 1, 10).into()),
                DiffCuboid::new((0, 2, 0).into(), (0, 10, 2).into()),
                DiffCuboid::new((0, 2, 3).into(), (0, 10, 10).into()),
                DiffCuboid::new((1, 0, 0).into(), (10, 1, 2).into()),
                DiffCuboid::new((1, 0, 3).into(), (10, 1, 10).into()),
                DiffCuboid::new((1, 2, 0).into(), (10, 10, 2).into()),
                DiffCuboid::new((1, 2, 9).into(), (10, 10, 10).into()),
                DiffCuboid::new((1, 8, 3).into(), (10, 10, 8).into()),
                DiffCuboid::new((7, 2, 3).into(), (10, 7, 8).into()),
            ]
        );

        let cube1 = DiffCuboid::new((10, 10, 10).into(), (20, 20, 20).into());
        let cube2 = DiffCuboid::new((5, 13, 12).into(), (25, 15, 16).into());
        let cubes = cube1.diff(&cube2);
        assert_eq!(
            &cubes,
            &[
                DiffCuboid::new((10, 10, 10).into(), (20, 12, 11).into()),
                DiffCuboid::new((10, 10, 12).into(), (20, 12, 20).into()),
                DiffCuboid::new((10, 13, 10).into(), (20, 20, 11).into()),
                DiffCuboid::new((10, 13, 17).into(), (20, 20, 20).into()),
                DiffCuboid::new((10, 16, 12).into(), (20, 20, 16).into())
            ]
        );
    }

    #[test]
    fn test_diff_no_overlap() {
        let cube1 = DiffCuboid::new((10, 10, 10).into(), (20, 20, 20).into());
        let cube2 = DiffCuboid::new((5, 0, 0).into(), (25, 5, 5).into());
        let cubes = cube1.diff(&cube2);
        assert_eq!(
            &cubes,
            &[DiffCuboid::new((10, 10, 10).into(), (20, 20, 20).into())]
        );

        let cubes = cube2.diff(&cube1);
        assert_eq!(
            &cubes,
            &[DiffCuboid::new((5, 0, 0).into(), (25, 5, 5).into())]
        );
    }
}
