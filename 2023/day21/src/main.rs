// https://adventofcode.com/2023/day/21

use std::{cell::RefCell, collections::HashSet, fmt::Display, rc::Rc};

use aoc::{
    grid::{DiffRectangle, Grid, GridBuilder, Point, PointDiff},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input(), 64));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// How many garden plots could be reached from the starting point in the given number of steps.
fn part1(input: AocInput, steps: usize) -> usize {
    let mut map = Map::from_input(input);
    for _ in 0..steps {
        map.walk();
        // println!("{i}:");
        // println!("{map}");
    }

    map.count()
}

// Use interpolation to do the same for a large number of steps.
// N.B. This does not work for the sample input, only the real input, because it depends on some
//      properties of that input, including the fact that 26501365 == 202300 * 131 + 65, where 131
//      is the size of the map.
fn part2(input: AocInput) -> usize {
    let mut map = Map::from_input(input);
    assert!(map.grid.width() == map.grid.height());
    let step_by = map.grid.width();
    let start = step_by / 2;
    let mut sequence = Vec::new();
    for i in 1..=(start + 3 * step_by) {
        map.walk();
        if (i - start) % step_by == 0 {
            println!("{i}: {}", map.count());
            sequence.push(map.count());
        }
    }

    println!("Computed result samples.");
    let mut extender = SequenceExtender::new(sequence);
    let mut last = 0;
    for _ in (4 * step_by..=26501365).step_by(131) {
        last = extender.extend();
    }

    last
}

#[derive(Clone)]
struct Sequence(Vec<usize>);

// Adapted from day 9
impl Sequence {
    fn diff(&self) -> Self {
        Self(
            self.0
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        )
    }

    fn diffs(&self) -> Vec<Self> {
        let mut result = vec![self.clone()];
        loop {
            let current = result.last().unwrap();
            let diff = current.diff();
            if diff.all_zeroes() {
                break;
            }

            result.push(diff);
        }

        result
    }

    fn all_zeroes(&self) -> bool {
        self.0.iter().all(|value| *value == 0)
    }
}

struct SequenceExtender {
    sequence: Sequence,
    diffs: Vec<Sequence>,
}

impl SequenceExtender {
    fn new(sequence: Vec<usize>) -> Self {
        let sequence = Sequence(sequence);
        let diffs = sequence.diffs();
        Self { sequence, diffs }
    }

    fn extend(&mut self) -> usize {
        let mut target = 0;
        for diff in self.diffs.iter_mut().rev() {
            let next = diff.0.last().unwrap();
            target += next;
            diff.0.push(target);
        }

        self.sequence.0.push(target);
        target
    }
}

#[derive(PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rock,
}

struct Map {
    grid: Grid<Tile>,
    reachable: HashSet<PointDiff>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rect = DiffRectangle::from_points(self.reachable.iter());
        let mut prev_row = 0;
        for pos in rect.points() {
            if pos.row() != prev_row {
                writeln!(f)?;
                prev_row = pos.row();
            }

            let ch = if self.reachable.contains(&pos) {
                'O'
            } else {
                match self.grid[self.grid.add_point_wrapped(Point::default(), pos).unwrap()] {
                    Tile::GardenPlot => '.',
                    Tile::Rock => '#',
                }
            };

            write!(f, "{ch}")?;
        }

        writeln!(f)
    }
}

impl Map {
    fn from_input(input: AocInput) -> Self {
        let start = Rc::new(RefCell::new(Point::default()));
        let start_clone = Rc::clone(&start);
        let grid = GridBuilder::from_input(input)
            .map(move |pos, cell| match cell {
                b'S' => {
                    *start_clone.borrow_mut() = pos;
                    Tile::GardenPlot
                }
                b'.' => Tile::GardenPlot,
                b'#' => Tile::Rock,
                _ => unreachable!(),
            })
            .build();

        let start_val = *start.borrow();
        let mut reachable = HashSet::new();
        reachable.insert(start_val.into_diff().unwrap());
        Self { grid, reachable }
    }

    fn walk(&mut self) {
        let new_reachable = self
            .reachable
            .iter()
            .flat_map(|pos| {
                pos.straight_neighbors().filter(|p| {
                    self.grid[self.grid.add_point_wrapped(Point::default(), *p).unwrap()]
                        == Tile::GardenPlot
                })
            })
            .collect();

        self.reachable = new_reachable;
    }

    fn count(&self) -> usize {
        self.reachable.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(16, part1(AocInput::from_sample(), 6));
    }
}
