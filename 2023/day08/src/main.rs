// https://adventofcode.com/2023/day/8

use std::collections::{hash_map::Entry, HashMap};

use aoc::{input::AocInput, iterator::InfiniteRepeatExt};
use text_io::scan;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Determine how many steps to go from AAA to ZZZ following the instructions.
fn part1(input: AocInput) -> usize {
    let map = Map::from_input(input);
    map.follow()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct Map {
    start: usize,
    end: usize,
    instructions: String,
    nodes: Vec<Node>,
}

impl Map {
    fn from_input(mut input: AocInput) -> Self {
        let instructions = input.next().unwrap();
        input.next().unwrap();
        let mut nodes = Vec::new();
        let mut name_map = NameMap::new();
        for line in input {
            let node = Node::from_str(&line, &mut name_map);
            if node.name >= nodes.len() {
                nodes.resize(node.name + 1, Default::default());
            }

            let name = node.name;
            nodes[name] = node;
        }

        Self {
            start: name_map.map["AAA"],
            end: name_map.map["ZZZ"],
            instructions,
            nodes,
        }
    }

    fn follow(&self) -> usize {
        let mut current = self.start;
        let mut steps = 0;
        for instruction in self.instructions.bytes().infinite_repeat() {
            if current == self.end {
                break;
            }

            current = match instruction {
                b'L' => self.nodes[current].left,
                b'R' => self.nodes[current].right,
                _ => unreachable!(),
            };

            steps += 1;
        }

        steps
    }
}

#[derive(Default, Clone)]
struct Node {
    name: usize,
    left: usize,
    right: usize,
}

impl Node {
    fn from_str(s: &str, name_map: &mut NameMap) -> Self {
        let name: String;
        let left: String;
        let right: String;
        scan!(s.bytes() => "{} = ({}, {})", name, left, right);
        Self {
            name: name_map.map(name),
            left: name_map.map(left),
            right: name_map.map(right),
        }
    }
}

struct NameMap {
    map: HashMap<String, usize>,
    next: usize,
}

impl NameMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            next: 0,
        }
    }

    fn map(&mut self, name: String) -> usize {
        return match self.map.entry(name) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let id = self.next;
                self.next += 1;
                e.insert(id);
                id
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
