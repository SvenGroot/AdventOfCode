use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    path::Path,
    str::FromStr,
};

use aoc::{
    aoc_input,
    dijkstra::{shortest_paths, Graph},
    get_input,
};
use text_io::scan;

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let mut pipes = PipeNetwork::from_file(path);
    pipes.collapse_graph();
    pipes.compute_reachable();
    for valve in pipes.valves.values() {
        println!("{:?}", valve);
    }
    println!("Searching");
    pipes.simulate()
}

fn part2(path: impl AsRef<Path>) -> usize {
    get_input(path).map(|_| 0).sum()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveName(u32);

impl FromStr for ValveName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = (s.as_bytes()[0] - b'A') as u32;
        let b = (s.as_bytes()[1] - b'A') as u32;
        Ok(ValveName(a * 1000 + b))
    }
}

impl Display for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = (self.0 / 1000) as u8 + b'A';
        let b = (self.0 % 1000) as u8 + b'A';
        write!(f, "{}{}", a as char, b as char)
    }
}

#[derive(Debug)]
struct Valve {
    name: ValveName,
    flow_rate: usize,
    neighbors: BTreeMap<ValveName, usize>,
    open: bool,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name: String;
        let flow_rate: usize;
        let (valve, neighbors) = s.split_once("; ").unwrap();
        scan!(valve.bytes() => "Valve {} has flow rate={}", name, flow_rate);
        let neighbors = neighbors
            .strip_prefix("tunnels lead to valves ")
            .or_else(|| neighbors.strip_prefix("tunnel leads to valve "))
            .unwrap();

        Ok(Self {
            name: name.parse().unwrap(),
            flow_rate,
            neighbors: neighbors
                .split(", ")
                .map(|nb| (nb.parse().unwrap(), 1))
                .collect(),
            open: false,
        })
    }
}

#[derive(Debug)]
struct PipeNetwork {
    valves: BTreeMap<ValveName, Valve>,
}

impl PipeNetwork {
    const TOTAL_TIME: usize = 30;

    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let valves = get_input(path)
            .map(|line| {
                let valve: Valve = line.parse().unwrap();
                (valve.name, valve)
            })
            .collect();

        Self { valves }
    }

    pub fn collapse_graph(&mut self) {
        let names: Vec<_> = self.valves.keys().copied().collect();
        for name in names {
            if name.0 != 0 {
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

    pub fn simulate(&mut self) -> usize {
        self.simulate_step(ValveName(0), 0, 0)
    }

    fn simulate_step(&mut self, current: ValveName, elapsed: usize, last_step: usize) -> usize {
        // Get the total flow rate this minute.
        let rate: usize = self.current_rate();

        let mut valve = self.valves.get_mut(&current).unwrap();
        valve.open = true;

        let remaining = Self::TOTAL_TIME - elapsed;
        let neighbors = valve.neighbors.clone();
        let max = neighbors
            .iter()
            .filter_map(|(&nb, &weight)| {
                let weight = weight + 1;
                let v = &self.valves[&nb];
                (!v.open && weight <= remaining)
                    .then(|| self.simulate_step(nb, elapsed + weight, weight))
            })
            .max();

        let flow = rate * last_step;
        if let Some(max) = max {
            self.valves.get_mut(&current).unwrap().open = false;
            flow + max
        } else {
            let rate = self.current_rate();
            self.valves.get_mut(&current).unwrap().open = false;
            flow + (remaining) * rate
        }
    }

    fn current_rate(&self) -> usize {
        self.valves
            .values()
            .filter_map(|v| v.open.then_some(v.flow_rate))
            .sum()
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
        assert_eq!(0, part2(aoc_sample_input()));
    }
}
