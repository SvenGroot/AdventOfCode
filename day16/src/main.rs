use std::{collections::HashMap, fmt::Display, path::Path, str::FromStr};

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
    pipes.simulate()
}

fn part2(path: impl AsRef<Path>) -> usize {
    get_input(path).map(|_| 0).sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveName(u32);

impl FromStr for ValveName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = (s.as_bytes()[0] - b'A') as u32;
        let b = (s.as_bytes()[1] - b'A') as u32;
        Ok(ValveName(a * 26 + b))
    }
}

impl Display for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = (self.0 / 26) as u8 + b'A';
        let b = (self.0 % 26) as u8 + b'A';
        write!(f, "{}{}", a as char, b as char)
    }
}

#[derive(Debug)]
struct Valve {
    name: ValveName,
    flow_rate: usize,
    neighbors: Vec<ValveName>,
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
                .map(|nb| nb.parse().unwrap())
                .collect(),
            open: false,
        })
    }
}

#[derive(Debug)]
struct PipeNetwork {
    valves: HashMap<ValveName, Valve>,
}

#[derive(Clone, Copy)]
struct ValveDistance(Option<ValveName>, f32, usize);

impl ValveDistance {
    pub fn max(&self, other: &Self) -> Self {
        if self.1 > other.1 {
            *self
        } else {
            *other
        }
    }
}

impl PipeNetwork {
    const TOTAL_TIME: usize = 30;

    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let valves = get_input(path)
            .map(|line| {
                let valve: Valve = line.parse().unwrap();
                (valve.name.clone(), valve)
            })
            .collect();

        Self { valves }
    }

    pub fn simulate(&mut self) -> usize {
        let mut elapsed = 0;
        let mut current = Some(ValveName(0));
        let mut total_flow = 0;
        while elapsed < Self::TOTAL_TIME {
            let rate;
            (rate, elapsed, current) = self.simulate_step(current, elapsed);
            if let Some(pos) = current {
                println!("Elapsed {}; rate: {}; position: {}", elapsed, rate, pos);
            } else {
                println!("Elapsed {}; rate: {}", elapsed, rate);
            }
            total_flow += rate;
        }

        total_flow
    }

    fn simulate_step(
        &mut self,
        current: Option<ValveName>,
        mut elapsed: usize,
    ) -> (usize, usize, Option<ValveName>) {
        // Get the total flow rate this minute.
        let rate: usize = self
            .valves
            .values()
            .filter_map(|v| v.open.then_some(v.flow_rate))
            .sum();

        if current.is_none() {
            let remaining = Self::TOTAL_TIME - elapsed;
            return (rate * remaining, Self::TOTAL_TIME, None);
        }

        let current = current.unwrap();

        let info = shortest_paths(self, &current);
        let max = info
            .iter()
            .filter_map(|(name, info)| {
                let valve = &self.valves[name];
                (info.distance > 0 && !valve.open && info.distance + 1 < Self::TOTAL_TIME - elapsed)
                    .then_some(ValveDistance(
                        Some(*name),
                        valve.flow_rate as f32 / (info.distance + 1) as f32,
                        info.distance,
                    ))
            })
            .fold(ValveDistance(None, f32::NEG_INFINITY, 0), |a, b| a.max(&b));

        // Time needed to get there and open the valve.
        let new_elapsed = max.2 + 1;
        elapsed += new_elapsed;
        let max_valve = max.0.map(|n| self.valves.get_mut(&n).unwrap());
        if let Some(v) = max_valve {
            v.open = true;
        }
        (rate * new_elapsed, elapsed, max.0)
    }
}

impl Graph<ValveName> for PipeNetwork {
    fn vertices(&self) -> std::collections::HashSet<ValveName> {
        self.valves.keys().copied().collect()
    }

    fn neighbors(&self, v: &ValveName) -> Vec<ValveName> {
        self.valves[v].neighbors.clone()
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
