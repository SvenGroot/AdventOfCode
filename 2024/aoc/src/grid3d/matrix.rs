use std::ops::Mul;

use super::PointDiff3D;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Matrix3D([[isize; 3]; 3]);

impl Matrix3D {
    pub const ROTATE_X: Matrix3D = Matrix3D([[1, 0, 0], [0, 0, -1], [0, 1, 0]]);
    pub const ROTATE_Y: Matrix3D = Matrix3D([[0, 0, 1], [0, 1, 0], [-1, 0, 0]]);
    pub const ROTATE_Z: Matrix3D = Matrix3D([[0, -1, 0], [1, 0, 0], [0, 0, 1]]);

    pub fn new(matrix: [[isize; 3]; 3]) -> Self {
        Self(matrix)
    }
}

impl Mul<PointDiff3D> for Matrix3D {
    type Output = PointDiff3D;

    fn mul(self, rhs: PointDiff3D) -> Self::Output {
        let matrix = ndarray::aview2(&self.0);
        let vector = ndarray::arr1(&[rhs.x(), rhs.y(), rhs.z()]);
        let result = matrix.dot(&vector);
        PointDiff3D::new(result[0], result[1], result[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pt = PointDiff3D::new(-1, -1, 1);
        let pt = Matrix3D::ROTATE_X * pt;
        let pt = Matrix3D::ROTATE_Y * pt;
        let pt = Matrix3D::ROTATE_Y * pt;
        assert_eq!(PointDiff3D::new(1, -1, 1), pt);
        let pt = PointDiff3D::new(5, 6, -4);
        let pt = Matrix3D::ROTATE_X * pt;
        let pt = Matrix3D::ROTATE_Y * pt;
        let pt = Matrix3D::ROTATE_Y * pt;
        assert_eq!(PointDiff3D::new(-5, 4, -6), pt);
    }
}
