use std::{fmt::Display, path::Path};

use aoc::{
    aoc_input, get_input, get_input_single, get_input_vec,
    grid::{Point, PointDiff},
    iterator::{infinite_repeat, InfiniteRepeat},
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    run(path, 2022, 10000)
}

fn part2(path: impl AsRef<Path>) -> usize {
    run(path, 1_000_000_000_000, 10_000_000)
}

fn run(path: impl AsRef<Path>, rounds: usize, prune_interval: usize) -> usize {
    let rocks = Rock::get_rocks();
    let jets = get_jets(path);
    let mut rocks_repeater = infinite_repeat(&rocks);
    let mut jets_repeater = infinite_repeat(&jets);
    let mut chamber = Chamber::new();
    chamber.drop_rocks(
        rounds,
        prune_interval,
        &mut rocks_repeater,
        &mut jets_repeater,
    )
}

#[derive(Clone)]
struct Rock {
    patterns: Vec<u8>,
    width: usize,
}

impl Rock {
    fn new(patterns: &[String]) -> Self {
        let width = patterns[0].len();
        let patterns = patterns
            .iter()
            .map(|p| u8::from_str_radix(p, 2).unwrap())
            .collect();
        Self { patterns, width }
    }

    fn get_rocks() -> Vec<Self> {
        let mut input = aoc_input();
        input.set_file_name("day17_rocks.txt");
        get_input_vec(input)
            .split(String::is_empty)
            .map(Rock::new)
            .collect()
    }
}

#[derive(Clone)]
struct Chamber {
    lines: Vec<u8>,
    top: usize,
    offset: usize,
}

impl Chamber {
    const WIDTH: usize = 7;

    fn new() -> Self {
        Self {
            lines: vec![0; 20],
            top: 0,
            offset: 0,
        }
    }

    fn drop_rocks(
        &mut self,
        rounds: usize,
        prune_interval: usize,
        rocks: &mut InfiniteRepeat<Rock>,
        jets: &mut InfiniteRepeat<PointDiff>,
    ) -> usize {
        for i in 0..rounds {
            if i % prune_interval == 0 {
                self.prune_bottom();
                println!("{}: {}", i, self.offset);
            }
            self.drop_rock(rocks, jets);
            // println!("{}", self);
        }

        self.top + self.offset
    }

    fn drop_rock(
        &mut self,
        rocks: &mut InfiniteRepeat<Rock>,
        jets: &mut InfiniteRepeat<PointDiff>,
    ) {
        let rock = rocks.next().unwrap();
        let mut rock_pos = Point::new(self.top + 3, 2);
        let height_needed = rock_pos.row() + rock.patterns.len();
        if self.lines.len() < height_needed {
            self.lines.resize(height_needed * 2, 0);
        }

        let mut done = false;
        while !done {
            //let mut c = self.clone();
            //c.place_rock(rock, rock_pos);
            // println!("{}", c);
            (rock_pos, done) = self.step_rock(rock, rock_pos, jets);
        }

        self.place_rock(rock, rock_pos);

        self.top = self.top.max(rock_pos.row() + rock.patterns.len());
    }

    fn place_rock(&mut self, rock: &Rock, rock_pos: Point) {
        for (index, &pattern) in rock.patterns.iter().enumerate() {
            let pattern = pattern << rock_pos.col();
            self.lines[index + rock_pos.row()] |= pattern;
        }
    }

    fn step_rock(
        &mut self,
        rock: &Rock,
        pos: Point,
        jets: &mut InfiniteRepeat<PointDiff>,
    ) -> (Point, bool) {
        let jet = *jets.next().unwrap();
        let new_pos = self.move_rock(rock, pos, jet).unwrap_or(pos);
        // Vertical is reversed from normal grid
        if let Some(pos) = self.move_rock(rock, new_pos, PointDiff::UP) {
            (pos, false)
        } else {
            (new_pos, true)
        }
    }

    fn move_rock(&mut self, rock: &Rock, pos: Point, dir: PointDiff) -> Option<Point> {
        let new_pos = pos.add_diff(dir)?;
        if new_pos.col() + rock.width > Self::WIDTH {
            return None;
        }

        // No collision with wall or floor, check existing rocks.
        if new_pos.row() <= self.top {
            rock.patterns
                .iter()
                .enumerate()
                .all(|(index, &p)| {
                    let p = p << new_pos.col();
                    self.lines[index + new_pos.row()] & p == 0
                })
                .then_some(new_pos)
        } else {
            Some(new_pos)
        }
    }

    fn prune_bottom(&mut self) {
        let mut combined = 0;
        let new_offset = self.lines.len()
            - self
                .lines
                .iter()
                .rev()
                .position(|&line| {
                    combined |= line;
                    combined == 0b01111111
                })
                .unwrap_or(self.lines.len());

        self.offset += new_offset;
        self.top -= new_offset;
        self.lines = self.lines.split_off(new_offset);
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.lines.iter().rev() {
            writeln!(f, "{:07b}", line.reverse_bits() >> 1)?
        }

        Ok(())
    }
}

fn get_jets(path: impl AsRef<Path>) -> Vec<PointDiff> {
    get_input_single(path)
        .bytes()
        .map(|dir| match dir {
            b'>' => PointDiff::RIGHT,
            b'<' => PointDiff::LEFT,
            _ => unreachable!("{}", dir as char),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3068, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1514285714288, part2(aoc_sample_input()));
    }
}
