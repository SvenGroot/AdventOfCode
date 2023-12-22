// https://adventofcode.com/2023/day/19

use std::{collections::HashMap, ops::Range, str::FromStr};

use aoc::input::AocInput;

mod parser;

const MAX: u64 = 4001;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Sum the ratings of all accepted parts.
fn part1(input: AocInput) -> u64 {
    let system = System::from_input(input);
    system.check_parts()
}

// Get the total amount of accepted rating combinations.
fn part2(input: AocInput) -> u64 {
    let system = System::from_input(input);
    system.workflows["in"].accepted_count(AcceptedRanges::all(), &system.workflows)
}

struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl System {
    fn from_input(input: AocInput) -> Self {
        let input = input.into_vec();
        let mut split = input.split(|line| line.is_empty());
        let workflows = split
            .next()
            .unwrap()
            .iter()
            .map(|line| {
                let workflow = Workflow::from_str(line).unwrap();
                (workflow.name.clone(), workflow)
            })
            .collect();

        let parts = split
            .next()
            .unwrap()
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();

        Self { workflows, parts }
    }

    fn check_parts(&self) -> u64 {
        self.parts
            .iter()
            .filter(|part| part.check_accepted(&self.workflows))
            .map(|part| part.total_rating())
            .sum()
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn run(&self, part: &Part) -> &str {
        self.rules
            .iter()
            .filter_map(|rule| rule.eval(part))
            .next()
            .unwrap()
    }

    fn accepted_count(
        &self,
        mut remaining: AcceptedRanges,
        workflows: &HashMap<String, Workflow>,
    ) -> u64 {
        self.rules
            .iter()
            .map(|rule| {
                let (rem, count) = rule.accepted_ranges(remaining.clone(), workflows);
                remaining = rem;
                count
            })
            .sum()
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, result) = parser::workflow(s).unwrap();
        Ok(result)
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<RuleCondition>,
    target: String,
}

impl Rule {
    fn eval(&self, part: &Part) -> Option<&str> {
        self.condition
            .as_ref()
            .map_or(true, |cond| cond.eval(part))
            .then_some(&self.target)
    }

    fn accepted_ranges(
        &self,
        mut input: AcceptedRanges,
        workflows: &HashMap<String, Workflow>,
    ) -> (AcceptedRanges, u64) {
        let mut remaining = AcceptedRanges::none();
        if let Some(condition) = &self.condition {
            let (passed, failed) = match condition.operation {
                Operation::GreaterThan => (condition.value + 1..MAX, 1..condition.value + 1),
                Operation::LessThan => (1..condition.value, condition.value..MAX),
            };

            remaining = input.clone();
            input.restrict_range(condition.field, passed);
            remaining.restrict_range(condition.field, failed);
        }

        let count = if self.target == "A" {
            input.count()
        } else if self.target == "R" {
            0
        } else {
            workflows[&self.target].accepted_count(input, workflows)
        };

        (remaining, count)
    }
}

#[derive(Debug)]
struct RuleCondition {
    field: char,
    operation: Operation,
    value: u64,
}

impl RuleCondition {
    fn eval(&self, part: &Part) -> bool {
        let value = part.ratings[&self.field];
        match self.operation {
            Operation::GreaterThan => value > self.value,
            Operation::LessThan => value < self.value,
        }
    }
}

#[derive(Debug)]
enum Operation {
    LessThan,
    GreaterThan,
}

struct Part {
    ratings: HashMap<char, u64>,
}

impl Part {
    fn total_rating(&self) -> u64 {
        self.ratings.values().sum()
    }

    fn check_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut name = "in";
        loop {
            let workflow = &workflows[name];
            name = workflow.run(self);

            if name == "A" {
                return true;
            } else if name == "R" {
                return false;
            }
        }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, result) = parser::part(s).unwrap();
        Ok(result)
    }
}

#[derive(Clone)]
struct AcceptedRanges(HashMap<char, Range<u64>>);

impl AcceptedRanges {
    fn all() -> Self {
        Self([('x', 1..MAX), ('m', 1..MAX), ('a', 1..MAX), ('s', 1..MAX)].into())
    }

    fn none() -> Self {
        Self([('x', 1..1), ('m', 1..1), ('a', 1..1), ('s', 1..1)].into())
    }

    fn restrict_range(&mut self, field: char, range: Range<u64>) {
        // Intersect the two ranges.
        let old_range = self.0.get_mut(&field).unwrap();
        *old_range = range.start.max(old_range.start)..range.end.min(old_range.end)
    }

    fn count(&self) -> u64 {
        self.0.values().map(|r| r.end - r.start).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(19114, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(167409079868000, part2(AocInput::from_sample()));
    }
}
