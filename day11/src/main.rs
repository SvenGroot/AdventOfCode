#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use aoc::{get_input, Lcm};
use std::{fmt::Display, mem::replace, str::FromStr};

fn main() {
    const PATH: &str = "input/day11.txt";
    println!("Part 1: {}", part1(PATH));
    println!("Part 2: {}", part2(PATH));
}

fn part1(path: &str) -> usize {
    let mut game = MonkeyGame::parse(path, 3);
    game.play(20);
    game.get_monkey_business()
}

fn part2(path: &str) -> usize {
    let mut game = MonkeyGame::parse(path, 1);
    game.play(10000);
    game.get_monkey_business()
}

#[derive(Clone)]
enum Operand {
    Old,
    Value(usize),
}

#[derive(Clone)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    operand: Operand,
    test: usize,
    targets: [usize; 2],
    inspect_count: usize,
}

impl Monkey {
    fn parse(iter: &mut impl Iterator<Item = String>) -> Option<Self> {
        let line = iter.next()?;
        // Skip blank line if there is one.
        let line = if line.is_empty() { iter.next()? } else { line };
        // Skip the monkey line.
        let line = iter.next()?;
        let items = line
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();

        let line = iter.next()?;
        let (operation, operand) = line
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split_once(' ')
            .unwrap();

        let op = match operation.as_bytes()[0] {
            b'*' => Operation::Multiply,
            b'+' => Operation::Add,
            _ => unreachable!(),
        };

        let operand = match operand {
            "old" => Operand::Old,
            value => Operand::Value(value.parse().unwrap()),
        };

        let line = iter.next()?;
        let test = line
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let line = iter.next()?;
        let if_true = line
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let line = iter.next()?;
        let if_false = line
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Some(Self {
            items,
            op,
            operand,
            test,
            targets: [if_false, if_true],
            inspect_count: 0,
        })
    }

    fn inspect_items(&mut self, worry_drop: usize, worry_max: usize) -> Vec<(usize, usize)> {
        let items = replace(&mut self.items, Vec::new());
        items
            .iter()
            .map(|worry| {
                self.inspect_count += 1;
                self.inspect_item(*worry, worry_drop, worry_max)
            })
            .collect()
    }

    fn inspect_item(&self, worry: usize, worry_drop: usize, worry_max: usize) -> (usize, usize) {
        let mut worry = self.do_operation(worry);
        worry /= worry_drop;
        worry %= worry_max;
        let target = self.targets[usize::from(worry % self.test == 0)];
        (worry, target)
    }

    fn do_operation(&self, worry: usize) -> usize {
        let operand = match self.operand {
            Operand::Old => worry,
            Operand::Value(value) => value,
        };

        match self.op {
            Operation::Add => worry + operand,
            Operation::Multiply => worry * operand,
        }
    }
}

struct MonkeyGame {
    monkeys: Vec<Monkey>,
    worry_drop: usize,
    worry_max: usize,
}

impl MonkeyGame {
    fn parse(path: &str, worry_drop: usize) -> Self {
        let mut iter = get_input(path);
        let mut monkeys = Vec::new();
        loop {
            if let Some(monkey) = Monkey::parse(&mut iter) {
                monkeys.push(monkey);
            } else {
                break;
            }
        }

        // Use the least common multiple of the division tests to keep the worry values low.
        // This way the division tests still work, while preventing overflow.
        let worry_max = monkeys.iter().map(|m| m.test).lcm().unwrap();

        Self {
            monkeys,
            worry_drop,
            worry_max,
        }
    }

    fn play(&mut self, rounds: u32) {
        for round in 0..rounds {
            for index in 0..self.monkeys.len() {
                let monkey = &mut self.monkeys[index];
                let items = monkey.inspect_items(self.worry_drop, self.worry_max);
                for (item, target) in items {
                    self.monkeys[target].items.push(item);
                }
            }

            println!("Round {}:", round);
            self.show();
        }
    }

    fn get_monkey_business(&self) -> usize {
        let mut monkeys = self.monkeys.clone();
        monkeys.sort_by(|l, r| r.inspect_count.cmp(&l.inspect_count));
        monkeys[0].inspect_count * monkeys[1].inspect_count
    }

    fn show(&self) {
        for (index, monkey) in self.monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", index, monkey.items);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "../input/sample/day11.txt";

    #[test]
    fn test_part1() {
        assert_eq!(10605, part1(PATH));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2713310158, part2(PATH));
    }
}
