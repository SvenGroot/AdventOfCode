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
    let caves = CaveGraph::from_input(input);
    *caves.allow_double_small.borrow_mut() = true;
    caves.find_paths()
}

#[derive(PartialEq, Eq)]
enum CaveKind {
    Big,
    Small(u8),
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
    fn should_visit(&self, allow_double_small: &mut bool) -> bool {
        let mut kind = self.kind.borrow_mut();
        match &mut *kind {
            CaveKind::Small(visit_count) => {
                if *visit_count == 0 || *visit_count == 1 && *allow_double_small {
                    *visit_count += 1;
                    if *visit_count == 2 {
                        *allow_double_small = false;
                    }

                    true
                } else {
                    false
                }
            }
            CaveKind::Big => true,
        }
    }

    fn unvisit(&self, allow_double_small: &mut bool) {
        let mut kind = self.kind.borrow_mut();
        if let CaveKind::Small(visit_count) = &mut *kind {
            *visit_count -= 1;
            if *visit_count == 1 {
                *allow_double_small = true;
            }
        }
    }
}

struct CaveGraph {
    graph: Graph<Cave>,
    allow_double_small: RefCell<bool>,
}

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

        Self {
            graph,
            allow_double_small: RefCell::new(false),
        }
    }

    fn find_paths(&self) -> usize {
        let start = &self.graph[self.graph.find_vertex("start").unwrap()];
        self.find_path_core(start)
    }

    fn find_path_core(&self, vertex: &GraphVertex<Cave>) -> usize {
        if vertex.value().name == "end" {
            return 1;
        }

        let mut count = 0;
        for nb in vertex.neighbors() {
            let nb = &self.graph[*nb];
            if nb
                .value()
                .should_visit(&mut self.allow_double_small.borrow_mut())
            {
                count += self.find_path_core(nb);
                nb.value()
                    .unvisit(&mut self.allow_double_small.borrow_mut());
            }
        }

        count
    }

    fn add_cave(graph: &mut Graph<Cave>, name: String) -> VertexId {
        if let Some(id) = graph.find_vertex(&name) {
            return id;
        }

        let kind = RefCell::new(if name.as_bytes()[0].is_ascii_uppercase() {
            CaveKind::Big
        } else {
            CaveKind::Small(if name == "start" { 2 } else { 0 })
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
        assert_eq!(3509, part2(AocInput::from_sample()));
    }
}
