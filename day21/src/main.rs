// https://adventofcode.com/2022/day/21
use std::{cell::RefCell, collections::HashMap, ops::Deref, path::Path};

use aoc::{aoc_input, get_input};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> isize {
    let monkeys: HashMap<_, _> = get_input(path).map(|line| Monkey::parse(&line)).collect();
    monkeys["root"].get_number(&monkeys)
}

fn part2(path: impl AsRef<Path>) -> usize {
    get_input(path).map(|_| 0).sum()
}

enum MonkeyJob {
    Number(isize),
    Operation {
        left: String,
        right: String,
        operator: char,
    },
}

struct Monkey(RefCell<MonkeyJob>);

impl Monkey {
    fn parse(s: &str) -> (String, Self) {
        let (name, job) = s.split_once(": ").unwrap();
        let splits: Vec<_> = job.split(' ').collect();
        if splits.len() == 1 {
            (
                name.into(),
                Monkey(RefCell::new(MonkeyJob::Number(job.parse().unwrap()))),
            )
        } else if splits.len() == 3 {
            (
                name.into(),
                Monkey(RefCell::new(MonkeyJob::Operation {
                    left: splits[0].into(),
                    right: splits[2].into(),
                    operator: splits[1].chars().next().unwrap(),
                })),
            )
        } else {
            unreachable!();
        }
    }

    fn get_number(&self, monkeys: &HashMap<String, Monkey>) -> isize {
        let job = self.0.borrow();
        let (value, replace) = match job.deref() {
            MonkeyJob::Number(value) => (*value, false),
            MonkeyJob::Operation {
                left,
                right,
                operator,
            } => {
                let left = monkeys[left].get_number(monkeys);
                let right = monkeys[right].get_number(monkeys);
                (
                    match *operator {
                        '+' => left + right,
                        '-' => left - right,
                        '*' => left * right,
                        '/' => left / right,
                        _ => unreachable!(),
                    },
                    true,
                )
            }
        };

        drop(job);
        if replace {
            *self.0.borrow_mut() = MonkeyJob::Number(value);
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(152, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(aoc_sample_input()));
    }
}
