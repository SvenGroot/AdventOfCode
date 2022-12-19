use std::{
    collections::HashMap,
    fmt::Display,
    hash::{BuildHasher, Hash, Hasher},
    path::Path,
};

use aoc::{
    aoc_input, get_input_single, get_input_vec,
    grid::{Point, PointDiff},
    iterator::{infinite_repeat, InfiniteRepeat},
};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    run(path, 2022)
}

fn part2(path: impl AsRef<Path>) -> usize {
    run(path, 1_000_000_000_000)
}

fn run(path: impl AsRef<Path>, rounds: usize) -> usize {
    let rocks = Rock::get_rocks();
    let jets = get_jets(path);
    let mut rocks_repeater = infinite_repeat(&rocks);
    let mut jets_repeater = infinite_repeat(&jets);
    let mut chamber = Chamber::new();
    chamber.drop_rocks(rounds, &mut rocks_repeater, &mut jets_repeater)
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
        rocks: &mut InfiniteRepeat<Rock>,
        jets: &mut InfiniteRepeat<PointDiff>,
    ) -> usize {
        let mut seen = HashMap::new();
        let mut i = 0;
        let mut check_cycles = true;
        while i < rounds {
            self.drop_rock(rocks, jets);
            if check_cycles && self.trim() {
                let mut hasher = seen.hasher().build_hasher();
                self.lines.hash(&mut hasher);
                let hash = hasher.finish();
                let key = (hash, rocks.index(), jets.index());
                if let Some((prev, offset)) = seen.get(&key) {
                    println!(
                        "Found cycle at {}: {} from {}: {}",
                        i, self.offset, prev, offset
                    );

                    let skip_size = i - prev;
                    let skip = ((rounds - i) / skip_size) - 1;
                    i += skip * skip_size + 1;
                    self.offset += skip * (self.offset - offset);
                    check_cycles = false;
                    println!("Skipping to {}: {}", i, self.offset);
                    continue;
                } else {
                    seen.insert(key, (i, self.offset));
                }
            }

            i += 1;
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

    fn trim(&mut self) -> bool {
        let mut new_offset = None;
        for (index, window) in self.lines.as_slice().windows(4).enumerate().rev() {
            let combined = window.iter().copied().reduce(|c, i| c | i).unwrap();
            if combined == 0b01111111 {
                new_offset = Some(index);
                break;
            }
        }

        if let Some(new_offset) = new_offset {
            let new_offset = new_offset + 1;
            self.offset += new_offset;
            self.top -= new_offset;
            self.lines = self.lines.split_off(new_offset);
            self.lines.truncate(self.top);
            true
        } else {
            false
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.lines.iter().take(self.top).rev() {
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
