// https://adventofcode.com/2023/day/20

use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Count the number of low and high pulses if the button is pushed 1000 times.
fn part1(input: AocInput) -> usize {
    let mut config = ModuleConfig::from_input(input);
    let mut state = State {
        pending: VecDeque::new(),
        high_count: 0,
        low_count: 0,
    };

    for _ in 0..1000 {
        config.push_button(&mut state);
    }

    state.high_count * state.low_count
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct ModuleConfig(HashMap<String, Module>);

impl ModuleConfig {
    fn from_input(input: AocInput) -> Self {
        let mut modules: HashMap<_, _> = input
            .parsed::<Module>()
            .map(|module| (module.name.clone(), module))
            .collect();

        let clone = modules.clone();
        for module in clone.values() {
            for output in &module.outputs {
                if let Some(m) = modules.get_mut(output) {
                    m.add_input(module.name.clone());
                }
            }
        }

        Self(modules)
    }

    fn push_button(&mut self, state: &mut State) {
        let broadcaster = self.0.get_mut("broadcaster").unwrap();
        state.low_count += 1;
        broadcaster.pulse("button", false, state);
        while let Some(next) = state.pending.pop_front() {
            if let Some(dest) = self.0.get_mut(&next.to) {
                dest.pulse(&next.from, next.high, state);
            }
        }
    }
}

#[derive(Clone)]
struct Module {
    name: String,
    kind: ModuleKind,
    outputs: Vec<String>,
}

impl Module {
    fn add_input(&mut self, name: String) {
        if let ModuleKind::Conjunction(inputs) = &mut self.kind {
            inputs.insert(name, false);
        }
    }

    fn pulse(&mut self, from: &str, high: bool, state: &mut State) {
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
                *inputs.get_mut(from).unwrap() = high;
                !inputs.values().all(|high| *high)
            }
        };

        if output_pulse {
            state.high_count += self.outputs.len();
        } else {
            state.low_count += self.outputs.len();
        }

        state
            .pending
            .extend(self.outputs.iter().map(|output| PendingPulse {
                from: self.name.clone(),
                to: output.clone(),
                high: output_pulse,
            }));
    }
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, outputs) = s.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ").map(|s| s.into()).collect();
        let (name, kind) = if let Some(name) = name.strip_prefix('%') {
            (name, ModuleKind::FlipFlop(false))
        } else if let Some(name) = name.strip_prefix('&') {
            (name, ModuleKind::Conjunction(HashMap::new()))
        } else {
            assert_eq!(name, "broadcaster");
            (name, ModuleKind::Broadcast)
        };

        Ok(Self {
            name: name.into(),
            kind,
            outputs,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleKind {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

struct PendingPulse {
    from: String,
    to: String,
    high: bool,
}

struct State {
    pending: VecDeque<PendingPulse>,
    high_count: usize,
    low_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(11687500, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
