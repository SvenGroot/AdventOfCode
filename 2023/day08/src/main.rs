// https://adventofcode.com/2023/day/8

use std::collections::{hash_map::Entry, HashMap};

use aoc::{input::AocInput, iterator::InfiniteRepeatExt, Lcm};
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

// Starting at multiple positions simultaneously, determine how many steps before they are all at
// an end.
fn part2(input: AocInput) -> usize {
    let map = Map::from_input(input);
    map.follow_many()
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
        self.find_path(self.start, Some(self.end))
    }

    fn follow_many(&self) -> usize {
        // It turns out that for every start node, if it reaches an end in N steps, it will then
        // reach that same end again in N steps. So, all we need is for each node to find the number
        // of steps before it reaches an end, and then use the lowest common multiple of all those
        // values.
        self.nodes
            .iter()
            .filter(|node| node.kind == NodeKind::Start)
            .map(|node| self.find_path(node.name, None))
            .lcm()
            .unwrap()
    }

    fn find_path(&self, start: usize, end: Option<usize>) -> usize {
        let mut current = start;
        let mut steps = 0;
        for instruction in self.instructions.bytes().infinite_repeat() {
            if let Some(end) = end {
                if current == end {
                    break;
                }
            } else if self.nodes[current].kind == NodeKind::End {
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
    kind: NodeKind,
}

impl Node {
    fn from_str(s: &str, name_map: &mut NameMap) -> Self {
        let name: String;
        let left: String;
        let right: String;
        scan!(s.bytes() => "{} = ({}, {})", name, left, right);

        // Used for part 2.
        let kind = match name.bytes().last().unwrap() {
            b'A' => NodeKind::Start,
            b'Z' => NodeKind::End,
            _ => NodeKind::Regular,
        };

        Self {
            name: name_map.map(name),
            left: name_map.map(left),
            right: name_map.map(right),
            kind,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum NodeKind {
    #[default]
    Regular,
    Start,
    End,
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
        assert_eq!(
            6,
            part2(AocInput::from_file(AocInput::get_custom_path(
                "day8_part2",
                true
            )))
        );
    }
}
