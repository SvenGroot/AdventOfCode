// https://adventofcode.com/2023/day/5

use std::{
    str::FromStr,
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Find the seed that maps to the lowest location.
fn part1(input: AocInput) -> usize {
    let almanac = Almanac::from_input(input.into_vec());
    almanac.seeds_to_location().min().unwrap()
}

// Same, but treat the list of seeds as [start, length] ranges.
// N.B.: Slow even in release mode (takes about 20 seconds on my machine).
fn part2(input: AocInput) -> usize {
    let almanac = Almanac::from_input(input.into_vec());
    almanac.seed_ranges_to_min_location()
}

struct Almanac {
    seeds: Vec<usize>,
    // Maps are stored in the order they should be chained together.
    maps: Vec<Vec<Range>>,
}

impl Almanac {
    fn from_input(input: Vec<String>) -> Self {
        let mut parts = input.split(|line| line.is_empty());
        let seeds = parts.next().unwrap()[0]
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|value| value.parse().unwrap())
            .collect();

        let maps = parts
            .map(|part| {
                part.iter()
                    .skip(1)
                    .map(|map| map.parse().unwrap())
                    .collect()
            })
            .collect();

        Self { seeds, maps }
    }

    fn seeds_to_location(&self) -> impl Iterator<Item = usize> + '_ {
        self.seeds.iter().map(|&seed| self.seed_to_location(seed))
    }

    fn seed_ranges_to_min_location(self) -> usize {
        let (sender, receiver) = mpsc::channel();
        let me = Arc::new(self);

        // Some basic threading to improve the speed. Could be improved further but unnecessary.
        for chunk in me.seeds.chunks(2) {
            let clone = Arc::clone(&me);
            let sender = sender.clone();
            let start = chunk[0];
            let length = chunk[1];
            thread::spawn(move || clone.seed_range_to_min_location(start, length, sender));
        }

        drop(sender);
        receiver.into_iter().min().unwrap()
    }

    fn seed_range_to_min_location(&self, start: usize, length: usize, sender: Sender<usize>) {
        let location = (start..start + length)
            .map(|seed| self.seed_to_location(seed))
            .min()
            .unwrap();

        sender.send(location).unwrap();
    }

    fn seed_to_location(&self, seed: usize) -> usize {
        let mut current = seed;
        for map in &self.maps {
            // The maps are too small to make binary search worth it.
            if let Some(range) = map.iter().find(|range| {
                current >= range.source_start && current < range.source_start + range.length
            }) {
                let offset = current - range.source_start;
                current = range.destination_start + offset;
            }
        }

        current
    }
}

struct Range {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        Ok(Self {
            destination_start: parts.next().unwrap().parse().unwrap(),
            source_start: parts.next().unwrap().parse().unwrap(),
            length: parts.next().unwrap().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2(AocInput::from_sample()));
    }
}
