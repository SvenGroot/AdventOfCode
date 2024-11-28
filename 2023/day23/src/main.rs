// https://adventofcode.com/2023/day/23

use aoc::{
    grid::{Grid, GridBuilder, Point, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Get the longest hike that doesn't visit the same tile twice.
fn part1(input: AocInput) -> usize {
    let mut map = TrailMap::from_input(input, true);
    map.longest_path()
}

// The same, but ignores slopes.
// N.B. Very slow, takes several minutes.
fn part2(input: AocInput) -> usize {
    let mut map = TrailMap::from_input(input, false);
    map.longest_path()
}

#[derive(PartialEq, Eq)]
enum TileKind {
    Path,
    Forest,
    Slope(PointDiff),
}

struct Tile {
    kind: TileKind,
    visited: bool,
}

struct TrailMap(Grid<Tile>);

impl TrailMap {
    fn from_input(input: AocInput, with_slopes: bool) -> Self {
        let grid = GridBuilder::from_input(input)
            .map(move |_, ch| Tile {
                kind: match ch {
                    b'.' => TileKind::Path,
                    b'#' => TileKind::Forest,
                    _ => {
                        if with_slopes {
                            TileKind::Slope(PointDiff::from_arrows(ch).unwrap())
                        } else {
                            TileKind::Path
                        }
                    }
                },
                visited: ch == b'#',
            })
            .build();

        Self(grid)
    }

    fn longest_path(&mut self) -> usize {
        let start = Point::new(0, 1);
        self.longest_path_core(start).unwrap()
    }

    fn longest_path_core(&mut self, pos: Point) -> Option<usize> {
        assert!(!self.0[pos].visited);
        self.0[pos].visited = true;
        let end = Point::new(self.0.height() - 1, self.0.width() - 2);
        let path_len = if pos == end {
            self.0[pos].visited = false;
            return Some(0);
        } else if let TileKind::Slope(dir) = self.0[pos].kind {
            let nb = pos + dir;
            if self.0[nb].visited {
                None
            } else {
                self.longest_path_core(nb)
            }
        } else {
            self.0
                .straight_neighbors(pos)
                .filter_map(|nb| {
                    if self.0[nb].visited {
                        return None;
                    }

                    self.longest_path_core(nb)
                })
                .max()
        };

        self.0[pos].visited = false;
        Some(path_len? + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(94, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(154, part2(AocInput::from_sample()));
    }
}
