// https://adventofcode.com/2021/day/12

use std::cell::RefCell;

use aoc::{
    graph::{Graph, GraphVertex, VertexId},
    input::AocInput,
};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

fn part1(input: AocInput) -> usize {
    let caves = CaveGraph::from_input(input);
    caves.find_paths()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

#[derive(PartialEq, Eq)]
enum CaveKind {
    Big,
    Small(bool),
}

struct Cave {
    name: String,
    kind: RefCell<CaveKind>,
}

impl PartialEq<str> for Cave {
    fn eq(&self, other: &str) -> bool {
        self.name == *other
    }
}

impl Cave {
    fn should_visit(&self) -> bool {
        let mut kind = self.kind.borrow_mut();
        match *kind {
            CaveKind::Small(false) => {
                *kind = CaveKind::Small(true);
                true
            }
            CaveKind::Big => true,
            _ => false,
        }
    }

    fn unvisit(&self) {
        let mut kind = self.kind.borrow_mut();
        if *kind == CaveKind::Small(true) {
            *kind = CaveKind::Small(false);
        }
    }
}

struct CaveGraph(Graph<Cave>);

impl CaveGraph {
    fn from_input(input: AocInput) -> Self {
        let mut graph = Graph::new();
        for line in input {
            let (from, to) = line.split_once('-').unwrap();
            let from = Self::add_cave(&mut graph, from.into());
            let to = Self::add_cave(&mut graph, to.into());
            graph.add_edge_by_id(from, to);
            graph.add_edge_by_id(to, from);
        }

        Self(graph)
    }

    fn find_paths(&self) -> usize {
        let start = &self.0[self.0.find_vertex("start").unwrap()];
        assert!(start.value().should_visit());
        self.find_path_core(start)
    }

    fn find_path_core(&self, vertex: &GraphVertex<Cave>) -> usize {
        // println!("> {}", vertex.value().name);
        if vertex.value().name == "end" {
            return 1;
        }

        let mut count = 0;
        for nb in vertex.neighbors() {
            let nb = &self.0[*nb];
            if nb.value().should_visit() {
                count += self.find_path_core(nb);
                nb.value().unvisit();
            }
        }

        // println!("< {}", vertex.value().name);
        count
    }

    fn add_cave(graph: &mut Graph<Cave>, name: String) -> VertexId {
        if let Some(id) = graph.find_vertex(&name) {
            return id;
        }

        let kind = RefCell::new(if name.as_bytes()[0].is_ascii_uppercase() {
            CaveKind::Big
        } else {
            CaveKind::Small(false)
        });

        graph.add_vertex(Cave { name, kind })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(226, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
