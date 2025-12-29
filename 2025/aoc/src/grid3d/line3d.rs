use thiserror::Error;

use super::{LineIterator3D, ParsePointError, Point3D, PointDiff3D};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Line3D {
    from: Point3D,
    to: Point3D,
}

impl Line3D {
    pub fn new(from: Point3D, to: Point3D) -> Self {
        Self { from, to }
    }

    pub fn points(&self) -> LineIterator3D {
        self.from.line_to(self.to).unwrap()
    }

    pub fn intersects(&self, other: &Line3D) -> bool {
        self.points().any(|p| other.points().any(|p2| p == p2))
    }

    pub fn parse(s: &str, separator: &str) -> Result<Self, ParseLine3DError> {
        let (from, to) = s
            .split_once(separator)
            .ok_or(ParseLine3DError::MissingSeparator)?;

        Ok(Self::new(from.parse()?, to.parse()?))
    }

    pub fn add_diff(&self, diff: PointDiff3D) -> Option<Self> {
        Some(Self {
            from: self.from.add_diff(diff)?,
            to: self.to.add_diff(diff)?,
        })
    }

    pub fn from(&self) -> Point3D {
        self.from
    }

    pub fn to(&self) -> Point3D {
        self.to
    }
}

#[derive(Error, Debug)]
pub enum ParseLine3DError {
    #[error("doesn't have a separator")]
    MissingSeparator,
    #[error("error parsing int")]
    ParsePointError(#[from] ParsePointError),
}
