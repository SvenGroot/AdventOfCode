// https://adventofcode.com/2025/day/1

use std::str::FromStr;

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Count number of times the dial reaches 0.
fn part1(input: AocInput) -> usize {
    let mut dial = 50usize;
    input
        .parsed::<Instruction>()
        .filter_map(|inst| {
            dial = inst.apply(dial);
            (dial == 0).then_some(())
        })
        .count()
}

// Count the number of times the dial passes 0 when applying all instructions.
fn part2(input: AocInput) -> usize {
    input
        .parsed::<Instruction>()
        .map(|inst| inst.distance)
        .fold((50isize, 0usize), |(current_dial, count), distance| {
            let new_dial = current_dial + distance;
            let mut passes = ((current_dial / 100) - (new_dial / 100)).unsigned_abs();
            if current_dial.signum() != new_dial.signum() && current_dial != 0 {
                // Adjust for crossing zero when moving from positive to negative or vice versa.
                passes += 1;
            }

            let new_count = count + passes;
            let wrapped_dial = new_dial.rem_euclid(100);
            (wrapped_dial, new_count)
        })
        .1
}

struct Instruction {
    distance: isize,
}

impl Instruction {
    fn apply(&self, dial: usize) -> usize {
        (dial as isize + self.distance).rem_euclid(100) as usize
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_char, dist_str) = s.split_at(1);
        let sign = match dir_char {
            "L" => -1,
            "R" => 1,
            _ => return Err(()),
        };
        let distance: isize = dist_str.parse().map_err(|_| ())?;

        Ok(Instruction {
            distance: sign * distance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(AocInput::from_sample()));
    }
}
