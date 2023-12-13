// https://adventofcode.com/2023/day/12

use std::{collections::HashMap, str::FromStr};

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Count the possible valid arrangements of damaged and operational springs.
fn part1(input: AocInput) -> usize {
    input
        .parsed::<SpringRow>()
        .map(SpringRow::count_arrangements)
        .sum()
}

// Unfold the map, then do the above.
fn part2(input: AocInput) -> usize {
    input
        .parsed::<SpringRow>()
        .map(|row| row.unfold().count_arrangements())
        .sum()
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<SpringState>,
    groups: Vec<usize>,
}

impl SpringRow {
    fn unfold(mut self) -> Self {
        let original_springs_len = self.springs.len();
        let original_groups_len = self.groups.len();
        for _ in 0..4 {
            self.springs.push(SpringState::Unknown);
            self.springs.extend_from_within(..original_springs_len);
            self.groups.extend_from_within(..original_groups_len);
        }

        self
    }

    fn count_arrangements(mut self) -> usize {
        let state = State {
            spring_index: 0,
            group_index: 0,
            current_group_size: 0,
        };

        let mut seen = HashMap::new();
        self.count_arrangements_core(state, &mut seen)
    }

    fn count_arrangements_core(
        &mut self,
        mut state: State,
        seen: &mut HashMap<State, usize>,
    ) -> usize {
        while state.spring_index < self.springs.len()
            && self.springs[state.spring_index] != SpringState::Unknown
        {
            if self.springs[state.spring_index] == SpringState::Operational {
                if !self.reset_group(&mut state) {
                    // Not a valid state
                    return 0;
                }
            } else {
                state.current_group_size += 1;
            }

            state.spring_index += 1;
        }

        if state.spring_index == self.springs.len() {
            // This is a valid arrangement if we've seen all groups.
            if self.reset_group(&mut state) && state.group_index == self.groups.len() {
                return 1;
            }

            return 0;
        }

        // No more modifying state from this point.
        let state = state;
        if let Some(count) = seen.get(&state) {
            return *count;
        }

        let target_len = self
            .groups
            .get(state.group_index)
            .copied()
            .unwrap_or_default();

        assert_eq!(self.springs[state.spring_index], SpringState::Unknown);
        let mut count = 0;
        if state.current_group_size == 0 || state.current_group_size == target_len {
            self.springs[state.spring_index] = SpringState::Operational;
            let mut new_state = State {
                spring_index: state.spring_index + 1,
                ..state
            };

            if new_state.current_group_size != 0 {
                new_state.current_group_size = 0;
                new_state.group_index += 1;
            }

            count += self.count_arrangements_core(new_state, seen);
        }

        if state.current_group_size < target_len {
            self.springs[state.spring_index] = SpringState::Damaged;
            let new_state = State {
                spring_index: state.spring_index + 1,
                current_group_size: state.current_group_size + 1,
                ..state
            };

            count += self.count_arrangements_core(new_state, seen);
        }

        self.springs[state.spring_index] = SpringState::Unknown;
        assert!(seen.insert(state, count).is_none());
        count
    }

    fn reset_group(&self, state: &mut State) -> bool {
        if state.current_group_size == 0 {
            return true;
        }

        let target_len = self
            .groups
            .get(state.group_index)
            .copied()
            .unwrap_or_default();

        if state.current_group_size != target_len {
            return false;
        }

        state.current_group_size = 0;
        state.group_index += 1;
        true
    }
}

impl FromStr for SpringRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s.split_once(' ').unwrap();
        let springs = springs
            .bytes()
            .map(|b| match b {
                b'.' => SpringState::Operational,
                b'#' => SpringState::Damaged,
                b'?' => SpringState::Unknown,
                _ => unreachable!(),
            })
            .collect();

        let groups = groups
            .split(',')
            .map(|group| group.parse().unwrap())
            .collect();

        Ok(Self { springs, groups })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    spring_index: usize,
    group_index: usize,
    current_group_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(525152, part2(AocInput::from_sample()));
    }
}
