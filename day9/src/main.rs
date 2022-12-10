#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use aoc::get_input;
use std::{collections::HashSet, str::FromStr};

fn main() {
    const PATH: &str = "input/day9.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

fn part1(path: &str) -> usize {
    simulate(path, 2)
}

fn part2(path: &str) -> usize {
    simulate(path, 10)
}

fn simulate(path: &str, knots: usize) -> usize {
    let mut visited = HashSet::new();
    let mut knots = vec![Position::new(); knots];
    visited.insert(*knots.last().unwrap());
    for line in get_input(path) {
        let amount = i32::from_str(&line[2..]).unwrap();
        for i in 0..amount {
            knots[0] = knots[0].mv(line.as_bytes()[0]);
            for i in 1..knots.len() {
                if !knots[i].is_adjacent(&knots[i - 1]) {
                    knots[i] = knots[i].move_toward(&knots[i - 1]);
                    visited.insert(*knots.last().unwrap());
                }
            }
        }
    }

    visited.len()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn draw_grid(min: i32, max: i32, knots: &[Position]) {
    for y in (min..max).rev() {
        for x in min..max {
            let mut ch = if (x, y) == (0, 0) { 's' } else { '.' };
            for (index, knot) in knots.iter().enumerate().rev() {
                if knot.x == x && knot.y == y {
                    ch = if index == 0 {
                        'H'
                    } else if index == knots.len() - 1 {
                        'T'
                    } else {
                        (index - '0' as usize) as u8 as char
                    };
                }
            }

            print!("{}", ch)
        }

        println!();
    }

    println!();
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn is_adjacent(&self, other: &Position) -> bool {
        (other.x - self.x).abs() <= 1 && (other.y - self.y).abs() <= 1
    }

    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn move_toward(&self, other: &Position) -> Self {
        let amount = (*other - *self).signum();
        *self + amount
    }

    fn mv(&self, dir: u8) -> Self {
        match dir {
            b'R' => Self {
                x: self.x + 1,
                y: self.y,
            },
            b'L' => Self {
                x: self.x - 1,
                y: self.y,
            },
            b'U' => Self {
                x: self.x,
                y: self.y + 1,
            },
            b'D' => Self {
                x: self.x,
                y: self.y - 1,
            },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "../input/sample/day9.txt";

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(PATH));
    }

    #[test]
    fn test_part2() {
        assert_eq!(36, part2("../input/sample/day9_2.txt"));
    }

    fn test_position() {}
}
