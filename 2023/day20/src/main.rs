// https://adventofcode.com/2023/day/20

use std::collections::{HashMap, VecDeque};

use aoc::{input::AocInput, Lcm, NameMap};

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2());
}

// Count the number of low and high pulses if the button is pushed 1000 times.
fn part1(input: AocInput) -> usize {
    let (mut config, broadcaster) = ModuleConfig::from_input(input, None);
    let mut state = State {
        pending: VecDeque::new(),
        high_count: 0,
        low_count: 0,
        rx_low_count: 0,
    };

    for _ in 0..1000 {
        config.push_button(&mut state, broadcaster);
    }

    state.high_count * state.low_count
}

// How many times to push the button until rx receives a single low pulse.
// See readme.
fn part2() -> usize {
    ["rr", "js", "zb", "bs"]
        .iter()
        .map(|count_name| run_part2(AocInput::from_input(), count_name))
        .lcm()
        .unwrap()
}

fn run_part2(input: AocInput, count_name: &str) -> usize {
    let (mut config, broadcaster) = ModuleConfig::from_input(input, Some(count_name));
    let mut state = State {
        pending: VecDeque::new(),
        high_count: 0,
        low_count: 0,
        rx_low_count: 0,
    };

    for i in 0..10000000000 {
        state.rx_low_count = 0;
        config.push_button(&mut state, broadcaster);
        if state.rx_low_count != 0 {
            return i + 1;
        }
    }

    unreachable!();
}

struct ModuleConfig(HashMap<usize, Module>);

impl ModuleConfig {
    fn from_input(input: AocInput, count_name: Option<&str>) -> (Self, usize) {
        let mut name_map = NameMap::new();
        let mut modules: HashMap<_, _> = input
            .map(|line| Module::from_str(&line, &mut name_map, count_name))
            .map(|module| (module.name, module))
            .collect();

        let clone = modules.clone();
        for module in clone.values() {
            for output in &module.outputs {
                if let Some(m) = modules.get_mut(output) {
                    m.add_input(module.name);
                }
            }
        }

        (Self(modules), name_map.map("broadcaster".into()))
    }

    fn push_button(&mut self, state: &mut State, broadcaster: usize) {
        let broadcaster = self.0.get_mut(&broadcaster).unwrap();
        state.low_count += 1;
        broadcaster.pulse(usize::MAX, false, state);
        while let Some(next) = state.pending.pop_front() {
            if let Some(dest) = self.0.get_mut(&next.to) {
                dest.pulse(next.from, next.high, state);
            }
        }
    }
}

#[derive(Clone)]
struct Module {
    name: usize,
    kind: ModuleKind,
    outputs: Vec<usize>,
    count: bool,
}

impl Module {
    fn add_input(&mut self, name: usize) {
        if let ModuleKind::Conjunction(inputs) = &mut self.kind {
            inputs.insert(name, false);
        }
    }

    fn pulse(&mut self, from: usize, high: bool, state: &mut State) {
        let output_pulse = match &mut self.kind {
            ModuleKind::Broadcast => high,
            ModuleKind::FlipFlop(state) => {
                if high {
                    return;
                } else {
                    *state = !*state;
                    *state
                }
            }
            ModuleKind::Conjunction(inputs) => {
                *inputs.get_mut(&from).unwrap() = high;
                !inputs.values().all(|high| *high)
            }
        };

        if output_pulse {
            if self.count {
                state.rx_low_count += 1;
            }
            state.high_count += self.outputs.len();
        } else {
            // if self.count {
            //     state.rx_low_count += 1;
            // }

            state.low_count += self.outputs.len();
        }

        state
            .pending
            .extend(self.outputs.iter().map(|output| PendingPulse {
                from: self.name,
                to: *output,
                high: output_pulse,
            }));
    }

    fn from_str(s: &str, map: &mut NameMap, count_name: Option<&str>) -> Self {
        let (name, outputs) = s.split_once(" -> ").unwrap();
        let outputs: Vec<_> = outputs.split(", ").map(|s| map.map(s.into())).collect();
        let (name, kind) = if let Some(name) = name.strip_prefix('%') {
            (name, ModuleKind::FlipFlop(false))
        } else if let Some(name) = name.strip_prefix('&') {
            (name, ModuleKind::Conjunction(HashMap::new()))
        } else {
            assert_eq!(name, "broadcaster");
            (name, ModuleKind::Broadcast)
        };

        let count = Some(name) == count_name;
        Self {
            name: map.map(name.into()),
            kind,
            outputs,
            count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleKind {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<usize, bool>),
}

struct PendingPulse {
    from: usize,
    to: usize,
    high: bool,
}

struct State {
    pending: VecDeque<PendingPulse>,
    high_count: usize,
    low_count: usize,
    rx_low_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(11687500, part1(AocInput::from_sample()));
    }
}
