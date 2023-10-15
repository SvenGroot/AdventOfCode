mod builder;
mod point;
mod pointdiff;
mod rect;
mod subgrid;

use std::{
    fmt::Display,
    num::NonZeroUsize,
    ops::{Index, IndexMut},
    slice::Iter,
};

pub use builder::GridBuilder;
pub use point::{Line, Neighbors, Point};
pub use pointdiff::{PointDiff, Rotation};
pub use rect::Rectangle;
pub use subgrid::SubGrid;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T: Clone> Grid<T> {
    pub fn new(height: NonZeroUsize, width: NonZeroUsize, value: T) -> Self {
        Self(vec![vec![value; width.into()]; height.into()])
    }

    pub fn from_points(points: Vec<Point>, empty_val: T, set_val: T) -> Self {
        let bounding_rect = Rectangle::from_points(points.iter());
        let mut result = Grid::new(
            bounding_rect.height().try_into().unwrap(),
            bounding_rect.width().try_into().unwrap(),
            empty_val,
        );
        for pos in &points {
            result[*pos] = set_val.clone();
        }

        result
    }

    pub fn from_string_points(
        points: impl Iterator<Item = impl AsRef<str>>,
        empty_val: T,
        set_val: T,
    ) -> Self {
        Self::from_points(
            points.map(|item| item.as_ref().parse().unwrap()).collect(),
            empty_val,
            set_val,
        )
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: Point) -> Option<&T> {
        self.at(index.row(), index.col())
    }

    pub fn get_mut(&mut self, index: Point) -> Option<&mut T> {
        self.at_mut(index.row(), index.col())
    }

    pub fn at(&self, row: usize, col: usize) -> Option<&T> {
        self.0.get(row)?.get(col)
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.0.get_mut(row)?.get_mut(col)
    }

    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> Grid<U> {
        let grid = self
            .0
            .iter()
            .map(|row| row.iter().map(&mut f).collect())
            .collect();

        Grid(grid)
    }

    pub fn rows(&self) -> impl Iterator<Item = Iter<'_, T>> {
        self.0.iter().map(|row| row.iter())
    }

    pub fn cols(&self) -> impl Iterator<Item = Scan<'_, T>> {
        (0..self.width()).map(|col| self.scan(Point::new(0, col), PointDiff::DOWN))
    }

    pub fn cells(&self) -> impl Iterator<Item = (Point, &T)> {
        self.0.iter().enumerate().flat_map(|(row, row_vec)| {
            row_vec
                .iter()
                .enumerate()
                .map(move |(col, cell)| (Point::new(row, col), cell))
        })
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (Point, &mut T)> {
        self.0.iter_mut().enumerate().flat_map(|(row, row_vec)| {
            row_vec
                .iter_mut()
                .enumerate()
                .map(move |(col, cell)| (Point::new(row, col), cell))
        })
    }

    pub fn bounding_rect(&self) -> Rectangle {
        Rectangle::new(
            Point::default(),
            Point::new(self.height() - 1, self.width() - 1),
        )
    }

    pub fn edge_cells(&self) -> impl Iterator<Item = (Point, &T)> {
        // This could obviously be done much more efficiently.
        let last_row = self.height() - 1;
        let last_col = self.width() - 1;
        self.cells().filter(move |(point, _)| {
            point.row() == 0
                || point.row() == last_row
                || point.col() == 0
                || point.col() == last_col
        })
    }

    pub fn edge_cells_mut(&mut self) -> impl Iterator<Item = (Point, &mut T)> {
        let last_row = self.height() - 1;
        let last_col = self.width() - 1;
        self.cells_mut().filter(move |(point, _)| {
            point.row() == 0
                || point.row() == last_row
                || point.col() == 0
                || point.col() == last_col
        })
    }

    pub fn straight_neighbors(&self, point: Point) -> impl Iterator<Item = Point> {
        self.neighbors(point, &PointDiff::STRAIGHT_NEIGHBORS)
    }

    pub fn all_neighbors(&self, point: Point) -> impl Iterator<Item = Point> {
        self.neighbors(point, &PointDiff::ALL_NEIGHBORS)
    }

    pub fn neighbors<'a>(
        &self,
        point: Point,
        neighbors: &'a [PointDiff],
    ) -> impl Iterator<Item = Point> + 'a {
        let height = self.height();
        let width = self.width();
        point
            .neighbors(neighbors)
            .filter(move |nb| nb.row() < height && nb.col() < width)
    }

    // Adds a diff to a point, only if the result is on the grid.
    pub fn add_point(&self, point: Point, diff: PointDiff) -> Option<Point> {
        let result = point.add_diff(diff)?;
        (result.row() < self.height() && result.col() < self.width()).then_some(result)
    }

    pub fn find_in_row(&self, row: usize, predicate: impl FnMut(&T) -> bool) -> Option<Point> {
        let col = self
            .scan(Point::new(row, 0), PointDiff::RIGHT)
            .position(predicate)?;

        Some(Point::new(row, col))
    }

    pub fn rfind_in_row(&self, row: usize, predicate: impl FnMut(&T) -> bool) -> Option<Point> {
        let col = self
            .scan(Point::new(row, self.width() - 1), PointDiff::LEFT)
            .position(predicate)?;

        Some(Point::new(row, self.width() - col - 1))
    }

    pub fn find_in_col(&self, col: usize, predicate: impl FnMut(&T) -> bool) -> Option<Point> {
        let row = self
            .scan(Point::new(0, col), PointDiff::DOWN)
            .position(predicate)?;

        Some(Point::new(row, col))
    }

    pub fn rfind_in_col(&self, col: usize, predicate: impl FnMut(&T) -> bool) -> Option<Point> {
        let row = self
            .scan(Point::new(self.height() - 1, col), PointDiff::UP)
            .position(predicate)?;

        Some(Point::new(self.height() - row - 1, col))
    }

    pub fn scan(&self, start: Point, direction: PointDiff) -> Scan<T> {
        Scan {
            grid: self,
            current: Some(start),
            direction,
        }
    }

    pub fn scan_mut(&mut self, start: Point, direction: PointDiff) -> ScanMut<T> {
        ScanMut {
            grid: self,
            current: Some(start),
            direction,
        }
    }

    pub fn shrink(&mut self, height: usize, width: usize) {
        self.0.truncate(height);
        for row in &mut self.0 {
            row.truncate(width);
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.row()][index.col()]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.row()][index.col()]
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let grid: Vec<Vec<T>> = iter.into_iter().collect();
        Self(grid)
    }
}

pub struct Scan<'a, T> {
    grid: &'a Grid<T>,
    current: Option<Point>,
    direction: PointDiff,
}

impl<'a, T> Iterator for Scan<'a, T> {
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

pub struct ScanMut<'a, T> {
    grid: &'a mut Grid<T>,
    current: Option<Point>,
    direction: PointDiff,
}

impl<'a, T> Iterator for ScanMut<'a, T> {
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
    fn test_straight_neighbors() {
        let grid = Grid::<u8>::new(10.try_into().unwrap(), 20.try_into().unwrap(), 0);
        let mut neighbors = grid.straight_neighbors(Point::new(2, 5));
        assert_eq!(Some(Point::new(1, 5)), neighbors.next());
        assert_eq!(Some(Point::new(2, 6)), neighbors.next());
        assert_eq!(Some(Point::new(3, 5)), neighbors.next());
        assert_eq!(Some(Point::new(2, 4)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.straight_neighbors(Point::new(0, 0));
        assert_eq!(Some(Point::new(0, 1)), neighbors.next());
        assert_eq!(Some(Point::new(1, 0)), neighbors.next());
        assert_eq!(None, neighbors.next());

        let mut neighbors = grid.straight_neighbors(Point::new(9, 19));
        assert_eq!(Some(Point::new(8, 19)), neighbors.next());
        assert_eq!(Some(Point::new(9, 18)), neighbors.next());
        assert_eq!(None, neighbors.next());
    }

    #[test]
    fn test_non_uniform() {
        let input = vec!["123".to_owned(), "12345".to_owned(), "12".to_owned()];
        let grid = GridBuilder::from_lines(input.into_iter())
            .extend(0, 0, b' ')
            .build();

        assert_eq!(5, grid.width());
        assert_eq!(3, grid.height());
        assert_eq!(&[b'1', b'2', b'3', b' ', b' '], grid.0[0].as_slice());
        assert_eq!(&[b'1', b'2', b'3', b'4', b'5'], grid.0[1].as_slice());
        assert_eq!(&[b'1', b'2', b' ', b' ', b' '], grid.0[2].as_slice());
    }
}
