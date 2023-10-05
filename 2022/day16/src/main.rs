#![allow(clippy::too_many_arguments)]
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    path::Path,
};

use aoc::{
    aoc_input,
    dijkstra::{shortest_paths, Graph},
    get_input,
};
use itertools::Itertools;
use text_io::scan;

const TOTAL_TIME: usize = 30;
const PART2_TIME: usize = 26;

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let mut pipes = PipeNetwork::from_file(path);
    pipes.collapse_graph();
    pipes.compute_reachable();
    println!("Searching");
    pipes.simulate(TOTAL_TIME).0
}

fn part2(path: impl AsRef<Path>) -> usize {
    let mut pipes = PipeNetwork::from_file(path);
    pipes.collapse_graph();
    pipes.compute_reachable();
    println!("Searching");
    let (_, best) = pipes.simulate(PART2_TIME);
    best.iter()
        .combinations(2)
        .filter_map(|c| {
            let (&path1, &flow1) = c[0];
            let (&path2, &flow2) = c[1];
            (path1 & path2 == 0).then_some(flow1 + flow2)
        })
        .max()
        .unwrap()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct ValveName(u64);

#[derive(Debug)]
struct Valve {
    name: ValveName,
    flow_rate: usize,
    neighbors: BTreeMap<ValveName, usize>,
    open: bool,
}

impl Valve {
    fn from_str(s: &str, name_map: &mut HashMap<String, u64>, next_name: &mut u64) -> Self {
        let name: String;
        let flow_rate: usize;
        let (valve, neighbors) = s.split_once("; ").unwrap();
        scan!(valve.bytes() => "Valve {} has flow rate={}", name, flow_rate);
        let neighbors = neighbors
            .strip_prefix("tunnels lead to valves ")
            .or_else(|| neighbors.strip_prefix("tunnel leads to valve "))
            .unwrap();

        let name = Self::get_name(Cow::Owned(name), name_map, next_name);

        Self {
            name,
            flow_rate,
            neighbors: neighbors
                .split(", ")
                .map(|nb| (Self::get_name(Cow::Borrowed(nb), name_map, next_name), 1))
                .collect(),
            open: false,
        }
    }

    fn get_name(
        name: Cow<str>,
        name_map: &mut HashMap<String, u64>,
        next_name: &mut u64,
    ) -> ValveName {
        let name = name_map
            .get(name.as_ref())
            .copied()
            .or_else(|| {
                let new_name = *next_name;
                name_map.insert(name.into_owned(), new_name);
                *next_name <<= 1;
                Some(new_name)
            })
            .unwrap();

        ValveName(name)
    }
}

#[derive(Debug)]
struct PipeNetwork {
    valves: BTreeMap<ValveName, Valve>,
    start: ValveName,
}

impl PipeNetwork {
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let mut name_map = HashMap::new();
        let mut next_name = 1;
        let valves = get_input(path)
            .map(|line| {
                let valve = Valve::from_str(&line, &mut name_map, &mut next_name);
                (valve.name, valve)
            })
            .collect();

        let start = name_map["AA"];
        Self {
            valves,
            start: ValveName(start),
        }
    }

    pub fn collapse_graph(&mut self) {
        let names: Vec<_> = self.valves.keys().copied().collect();
        for name in names {
            if name != self.start {
                self.collapse_node(name)
            }
        }
    }

    fn collapse_node(&mut self, name: ValveName) {
        let valve = &self.valves[&name];
        if valve.flow_rate != 0 {
            return;
        }

        let neighbors = valve.neighbors.clone();
        for (nb_name, weight) in &neighbors {
            let nb = self.valves.get_mut(nb_name).unwrap();
            nb.neighbors.remove(&name).unwrap();
            nb.neighbors
                .extend(neighbors.iter().filter_map(|(nb_name2, weight2)| {
                    (*nb_name != *nb_name2).then_some((*nb_name2, weight + weight2))
                }));
        }

        self.valves.remove(&name).unwrap();
    }

    pub fn compute_reachable(&mut self) {
        let names: Vec<_> = self.valves.keys().copied().collect();
        for name in names {
            self.compute_reachable_node(name)
        }
    }

    fn compute_reachable_node(&mut self, name: ValveName) {
        let info = shortest_paths(self, &name);
        let valve = self.valves.get_mut(&name).unwrap();
        valve.neighbors = info
            .iter()
            .filter_map(|(name, info)| (info.distance > 0).then_some((*name, info.distance)))
            .collect();
    }

    pub fn simulate(&mut self, time: usize) -> (usize, BTreeMap<u64, usize>) {
        let mut best = BTreeMap::new();
        let overall_best = self.simulate_step(self.start, time, 0, 0, 0, 0, &mut best);
        (overall_best, best)
    }

    fn simulate_step(
        &mut self,
        current: ValveName,
        remaining: usize,
        last_step: usize,
        mut flow: usize,
        mut rate: usize,
        mut opened: u64,
        best: &mut BTreeMap<u64, usize>,
    ) -> usize {
        // Get the total flow rate this minute.
        flow += rate * last_step;

        let valve = self.valves.get_mut(&current).unwrap();
        valve.open = true;
        rate += valve.flow_rate;

        // Don't include start in the bitmap.
        if current != self.start {
            opened |= current.0;
        }

        let current_best = best.get(&opened).copied().unwrap_or(0);
        let current_value = flow + rate * remaining;
        if current_value > current_best {
            best.insert(opened, current_value);
        }

        let neighbors = valve.neighbors.clone();
        let max = neighbors
            .iter()
            .filter_map(|(&nb, &weight)| {
                let weight = weight + 1;
                let v = &self.valves[&nb];
                (!v.open && weight <= remaining).then(|| {
                    self.simulate_step(nb, remaining - weight, weight, flow, rate, opened, best)
                })
            })
            .max();

        self.valves.get_mut(&current).unwrap().open = false;
        if let Some(max) = max {
            max
        } else {
            flow + (remaining) * rate
        }
    }
}

impl Graph<ValveName> for PipeNetwork {
    fn vertices(&self) -> std::collections::HashSet<ValveName> {
        self.valves.keys().copied().collect()
    }

    fn neighbors(&self, v: &ValveName) -> Vec<(ValveName, usize)> {
        self.valves[v]
            .neighbors
            .iter()
            .map(|(name, weight)| (*name, *weight))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1651, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1707, part2(aoc_sample_input()));
    }
}
