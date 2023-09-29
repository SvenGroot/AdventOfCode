// https://adventofcode.com/2022/day/21
use std::{cell::RefCell, collections::HashMap, ops::Deref, path::Path};

use aoc::{aoc_input, get_input};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

// What number does the monkey named "root" yell.
fn part1(path: impl AsRef<Path>) -> isize {
    let monkeys: HashMap<_, _> = get_input(path).map(|line| Monkey::parse(&line)).collect();
    let result = monkeys["root"].get_number(&monkeys).unwrap();
    result
}

// What number does "humn" have to yell so that "root" has left == right.
fn part2(path: impl AsRef<Path>) -> isize {
    let mut monkeys: HashMap<_, _> = get_input(path).map(|line| Monkey::parse(&line)).collect();
    monkeys.insert("humn".into(), Monkey(RefCell::new(MonkeyJob::Unknown)));
    let root = &monkeys["root"];
    let result = root.get_number(&monkeys);
    assert!(result.is_none());
    let job = root.0.borrow();
    let (known, unknown) = match job.deref() {
        MonkeyJob::Operation {
            left: OperandValue::Number(known),
            right: OperandValue::Unresolved(unknown),
            ..
        } => (*known, unknown),
        MonkeyJob::Operation {
            left: OperandValue::Unresolved(unknown),
            right: OperandValue::Number(known),
            ..
        } => (*known, unknown),
        _ => unreachable!(),
    };

    monkeys[unknown].resolve_unknown(&monkeys, known)
}

enum MonkeyJob {
    Number(isize),
    Operation {
        left: OperandValue,
        right: OperandValue,
        operator: char,
    },
    Unknown,
}

#[derive(Clone)]
enum OperandValue {
    Number(isize),
    Unresolved(String),
}

impl OperandValue {
    fn get_number(&self, monkeys: &HashMap<String, Monkey>) -> Option<isize> {
        match self {
            OperandValue::Number(value) => Some(*value),
            OperandValue::Unresolved(name) => monkeys[name].get_number(monkeys),
        }
    }
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
                    left: OperandValue::Unresolved(splits[0].into()),
                    right: OperandValue::Unresolved(splits[2].into()),
                    operator: splits[1].chars().next().unwrap(),
                })),
            )
        } else {
            unreachable!();
        }
    }

    fn get_number(&self, monkeys: &HashMap<String, Monkey>) -> Option<isize> {
        let job = self.0.borrow();
        let (value, replace) = match job.deref() {
            MonkeyJob::Unknown => (None, None),
            MonkeyJob::Number(value) => (Some(*value), None),
            MonkeyJob::Operation {
                left,
                right,
                operator,
            } => {
                let left_val = left.get_number(monkeys);
                let right_val = right.get_number(monkeys);
                match (left_val, right_val) {
                    (Some(left), Some(right)) => {
                        let result = match *operator {
                            '+' => left + right,
                            '-' => left - right,
                            '*' => left * right,
                            '/' => left / right,
                            _ => unreachable!(),
                        };
                        (Some(result), Some(MonkeyJob::Number(result)))
                    }
                    (Some(left), None) => (
                        None,
                        Some(MonkeyJob::Operation {
                            left: OperandValue::Number(left),
                            right: right.clone(),
                            operator: *operator,
                        }),
                    ),
                    (None, Some(right)) => (
                        None,
                        Some(MonkeyJob::Operation {
                            left: left.clone(),
                            right: OperandValue::Number(right),
                            operator: *operator,
                        }),
                    ),
                    _ => unreachable!(),
                }
            }
        };

        drop(job);
        if let Some(new_value) = replace {
            *self.0.borrow_mut() = new_value
        }

        value
    }

    fn resolve_unknown(&self, monkeys: &HashMap<String, Monkey>, expected: isize) -> isize {
        let job = self.0.borrow();
        let (known, unknown, is_left, operator) = match job.deref() {
            MonkeyJob::Operation {
                left: OperandValue::Number(known),
                right: OperandValue::Unresolved(unknown),
                operator,
            } => (*known, unknown, false, *operator),
            MonkeyJob::Operation {
                left: OperandValue::Unresolved(unknown),
                right: OperandValue::Number(known),
                operator,
            } => (*known, unknown, true, *operator),
            MonkeyJob::Unknown => return expected,
            _ => unreachable!(),
        };

        let x = match (operator, is_left) {
            ('+', _) => expected - known, // known + x = expected or x + num = expected
            ('-', false) => known - expected, // known - x = expected
            ('-', true) => expected + known, // x - known = expected
            ('*', _) => expected / known, // known * x = expected or reverse
            ('/', false) => known / expected, // known / x = expected
            ('/', true) => known * expected, // x / known = expected
            _ => unreachable!(),
        };

        monkeys[unknown].resolve_unknown(monkeys, x)
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
        assert_eq!(301, part2(aoc_sample_input()));
    }
}
