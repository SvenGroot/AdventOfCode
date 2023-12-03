// https://adventofcode.com/2023/day/2

use std::str::FromStr;

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Determine the sum of the IDs of all games that are possible with a bag containing the specified
// cubes.
fn part1(input: AocInput) -> usize {
    let bag = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .parsed::<Game>()
        .filter_map(|game| game.is_possible(&bag).then_some(game.id))
        .sum()
}

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct Game {
    id: usize,
    grabs: Vec<CubeSet>,
}

impl Game {
    fn is_possible(&self, bag: &CubeSet) -> bool {
        self.grabs
            .iter()
            .all(|grab| grab.red <= bag.red && grab.blue <= bag.blue && grab.green <= bag.green)
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").unwrap();
        let (id, grabs) = s.split_once(": ").unwrap();
        Ok(Game {
            id: id.parse().unwrap(),
            grabs: grabs
                .split("; ")
                .map(|grab| grab.parse().unwrap())
                .collect(),
        })
    }
}

#[derive(Default)]
struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}

impl FromStr for CubeSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = CubeSet::default();
        for ball in s.split(", ") {
            let (count, color) = ball.split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match color {
                "blue" => set.blue = count,
                "red" => set.red = count,
                "green" => set.green = count,
                _ => unreachable!(),
            }
        }

        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
