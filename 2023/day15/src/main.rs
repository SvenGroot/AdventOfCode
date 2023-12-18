// https://adventofcode.com/2023/day/15

use std::str::FromStr;

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Apply the HASH algorithm to every step in the sequence.
fn part1(input: AocInput) -> usize {
    input.single_line().split(',').map(hash_str).sum()
}

// Sort the lenses into boxes and get the total focal power.
fn part2(input: AocInput) -> usize {
    let mut boxes = Vec::new();
    boxes.resize_with(256, LensBox::default);
    for lens in input.single_line_parsed::<Lens>(',') {
        let index = lens.box_index();
        match lens.operation {
            Operation::Add(_) => boxes[index].add(lens),
            Operation::Remove => boxes[index].remove(&lens.label),
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(index, b)| {
            b.0.iter()
                .enumerate()
                .map(|(slot, lens)| lens.focal_length(index + 1, slot + 1))
                .sum::<usize>()
        })
        .sum()
}

fn hash_str(s: &str) -> usize {
    s.bytes()
        .fold(0, |current, b| ((current + b as usize) * 17) % 256)
}

#[derive(Default, Debug)]
struct LensBox(Vec<Lens>);

impl LensBox {
    fn add(&mut self, lens: Lens) {
        if let Some(existing) = self.0.iter_mut().find(|l| l.label == lens.label) {
            *existing = lens;
        } else {
            self.0.push(lens);
        }
    }

    fn remove(&mut self, label: &str) {
        self.0.retain(|lens| lens.label != label);
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    operation: Operation,
}

impl Lens {
    fn box_index(&self) -> usize {
        hash_str(&self.label)
    }
}

impl Lens {
    fn focal_length(&self, box_index: usize, slot: usize) -> usize {
        let Operation::Add(focal_length) = self.operation else {
            unreachable!();
        };

        box_index * slot * focal_length
    }
}

impl FromStr for Lens {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((label, focal)) = s.split_once('=') {
            Ok(Self {
                label: label.into(),
                operation: Operation::Add(focal.parse().unwrap()),
            })
        } else {
            let label = s.strip_suffix('-').unwrap();
            Ok(Self {
                label: label.into(),
                operation: Operation::Remove,
            })
        }
    }
}

#[derive(Debug)]
enum Operation {
    Remove,
    Add(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1320, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(145, part2(AocInput::from_sample()));
    }
}
