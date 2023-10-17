mod cube;
mod matrix;
mod point3d;
mod pointdiff3d;

use std::{
    num::NonZeroUsize,
    ops::{Index, IndexMut},
};

pub use cube::DiffCube;
pub use matrix::Matrix3D;
pub use point3d::{Line, Neighbors3D, ParsePointError, Point3D};
pub use pointdiff3d::PointDiff3D;

#[derive(Clone)]
pub struct Grid3D<T: Clone>(Vec<Vec<Vec<T>>>);

impl<T: Clone> Grid3D<T> {
    pub fn new(height: NonZeroUsize, width: NonZeroUsize, depth: NonZeroUsize, value: T) -> Self {
        Self(vec![
            vec![vec![value; width.into()]; height.into()];
            depth.into()
        ])
    }

    pub fn width(&self) -> usize {
        self.0[0][0].len()
    }

    pub fn height(&self) -> usize {
        self.0[0].len()
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: Point3D) -> Option<&T> {
        self.0.get(index.x())?.get(index.y())?.get(index.z())
    }

    pub fn get_mut(&mut self, index: Point3D) -> Option<&mut T> {
        self.0
            .get_mut(index.x())?
            .get_mut(index.y())?
            .get_mut(index.z())
    }

    pub fn map<U: Clone>(&self, mut f: impl FnMut(&T) -> U) -> Grid3D<U> {
        let grid = self
            .0
            .iter()
            .map(|x| x.iter().map(|y| y.iter().map(&mut f).collect()).collect())
            .collect();

        Grid3D(grid)
    }

    pub fn cells(&self) -> impl Iterator<Item = (Point3D, &T)> {
        self.0.iter().enumerate().flat_map(|(x, y_vec)| {
            y_vec.iter().enumerate().flat_map(move |(y, z_vec)| {
                z_vec
                    .iter()
                    .enumerate()
                    .map(move |(z, cell)| (Point3D::new(x, y, z), cell))
            })
        })
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (Point3D, &mut T)> {
        self.0.iter_mut().enumerate().flat_map(|(x, y_vec)| {
            y_vec.iter_mut().enumerate().flat_map(move |(y, z_vec)| {
                z_vec
                    .iter_mut()
                    .enumerate()
                    .map(move |(z, cell)| (Point3D::new(x, y, z), cell))
            })
        })
    }

    pub fn straight_neighbors(&self, point: Point3D) -> impl Iterator<Item = Point3D> + '_ {
        point
            .straight_neighbors()
            .filter(|nb| nb.x() < self.height() && nb.y() < self.width() && nb.z() < self.depth())
    }

    pub fn scan(&self, start: Point3D, direction: PointDiff3D) -> Scan3D<T> {
        Scan3D {
            grid: self,
            current: Some(start),
            direction,
        }
    }

    pub fn scan_mut(&mut self, start: Point3D, direction: PointDiff3D) -> ScanMut3D<T> {
        ScanMut3D {
            grid: self,
            current: Some(start),
            direction,
        }
    }
}

impl<T: Clone> Index<Point3D> for Grid3D<T> {
    type Output = T;

    fn index(&self, index: Point3D) -> &Self::Output {
        &self.0[index.x()][index.y()][index.z()]
    }
}

impl<T: Clone> IndexMut<Point3D> for Grid3D<T> {
    fn index_mut(&mut self, index: Point3D) -> &mut Self::Output {
        &mut self.0[index.x()][index.y()][index.z()]
    }
}

pub struct Scan3D<'a, T: Clone> {
    grid: &'a Grid3D<T>,
    current: Option<Point3D>,
    direction: PointDiff3D,
}

impl<'a, T: Clone> Iterator for Scan3D<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.current?)?;
        self.current = if let Some(p) = self.current {
            p.add_diff(self.direction)
        } else {
            None
        };

        Some(result)
    }
}

pub struct ScanMut3D<'a, T: Clone> {
    grid: &'a mut Grid3D<T>,
    current: Option<Point3D>,
    direction: PointDiff3D,
}

impl<'a, T: Clone> Iterator for ScanMut3D<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get_mut(self.current?)?;
        self.current = if let Some(p) = self.current {
            p.add_diff(self.direction)
        } else {
            None
        };

        // Work around borrow checker limitation.
        unsafe { Some(&mut *(result as *mut T)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_straight_neighbors() {
        let mut neighbors = Point3D::new(2, 5, 9).straight_neighbors();
        assert_eq!(Some(Point3D::new(1, 5, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 6, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(3, 5, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 4, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 5, 10)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 5, 8)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = Point3D::new(0, 0, 0).straight_neighbors();
        assert_eq!(Some(Point3D::new(0, 1, 0)), neighbors.next());
        assert_eq!(Some(Point3D::new(1, 0, 0)), neighbors.next());
        assert_eq!(Some(Point3D::new(0, 0, 1)), neighbors.next());
        assert_eq!(None, neighbors.next());
    }

    #[test]
    fn test_grid_straight_neighbors() {
        let grid = Grid3D::<u8>::new(
            10.try_into().unwrap(),
            20.try_into().unwrap(),
            30.try_into().unwrap(),
            0,
        );

        let mut neighbors = grid.straight_neighbors(Point3D::new(2, 5, 9));
        assert_eq!(Some(Point3D::new(1, 5, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 6, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(3, 5, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 4, 9)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 5, 10)), neighbors.next());
        assert_eq!(Some(Point3D::new(2, 5, 8)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.straight_neighbors(Point3D::new(0, 0, 0));
        assert_eq!(Some(Point3D::new(0, 1, 0)), neighbors.next());
        assert_eq!(Some(Point3D::new(1, 0, 0)), neighbors.next());
        assert_eq!(Some(Point3D::new(0, 0, 1)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.straight_neighbors(Point3D::new(9, 19, 29));
        assert_eq!(Some(Point3D::new(8, 19, 29)), neighbors.next());
        assert_eq!(Some(Point3D::new(9, 18, 29)), neighbors.next());
        assert_eq!(Some(Point3D::new(9, 19, 28)), neighbors.next());
        assert_eq!(None, neighbors.next());
    }
}
