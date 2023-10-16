// https://adventofcode.com/2021/day/14

use std::str::{self, FromStr};

use aoc::{input::AocInput, iterator::IntoVec};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Simulate polymer pair insertion for 10 steps.
fn part1(input: AocInput) -> usize {
    let polymer = Polymer::from_input(input);
    polymer.grow(10)
}

// Simulate polymer pair insertion for 40 steps.
fn part2(input: AocInput) -> usize {
    let polymer = Polymer::from_input(input);
    polymer.grow(40)
}

struct Polymer {
    template: Vec<u8>,
    raw_rules: Vec<RawRule>,
    rules: Vec<Rule>,
}

impl Polymer {
    fn from_input(mut input: AocInput) -> Self {
        let template = input.next().unwrap();
        input.next();
        let raw_rules = input.parsed::<RawRule>().into_vec();
        let rules = raw_rules
            .iter()
            .map(|rule| rule.to_rule(&raw_rules))
            .collect();

        Self {
            template: template.into(),
            raw_rules,
            rules,
        }
    }

    fn grow(&self, steps: usize) -> usize {
        let mut elements = [0usize; 26];

        // Initialize with counts already in the string.
        for elt in &self.template {
            elements[(*elt - b'A') as usize] += 1;
        }

        for pair in self.template.windows(2) {
            let pair = str::from_utf8(pair).unwrap();
            let rule_index = self
                .raw_rules
                .iter()
                .position(|rule| rule.pair == pair)
                .unwrap();

            println!("Processing {pair}");
            self.process_rule(&mut elements, rule_index, steps - 1)
        }

        let max = elements.iter().max().unwrap();
        let min = elements.iter().filter(|elt| **elt != 0).min().unwrap();
        println!("Min: {min}, max: {max}");
        max - min
    }

    fn process_rule(&self, elements: &mut [usize], rule_index: usize, steps: usize) {
        let rule = &self.rules[rule_index];
        elements[rule.element_index] += 1;
        if steps == 0 {
            return;
        }

        self.process_rule(elements, rule.new_pairs.0, steps - 1);
        self.process_rule(elements, rule.new_pairs.1, steps - 1);
    }
}

struct RawRule {
    pair: String,
    insert: u8,
}

impl RawRule {
    fn to_rule(&self, rules: &[RawRule]) -> Rule {
        let pair1 = Self::get_pair_index(rules, self.pair.as_bytes()[0], self.insert);
        let pair2 = Self::get_pair_index(rules, self.insert, self.pair.as_bytes()[1]);
        Rule {
            new_pairs: (pair1, pair2),
            element_index: (self.insert - b'A') as usize,
        }
    }

    fn get_pair_index(rules: &[RawRule], first: u8, last: u8) -> usize {
        let pair = [first, last];
        let pair = str::from_utf8(&pair).unwrap();
        rules.iter().position(|rule| rule.pair == pair).unwrap()
    }
}

impl FromStr for RawRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pair, insert) = s.split_once(" -> ").unwrap();
        Ok(Self {
            pair: pair.to_string(),
            insert: insert.as_bytes()[0],
        })
    }
}

struct Rule {
    new_pairs: (usize, usize),
    element_index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1588, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2188189693529, part2(AocInput::from_sample()));
    }
}
