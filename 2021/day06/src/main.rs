// https://adventofcode.com/2021/day/6

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// How many lanternfish after 80 days.
fn part1(input: AocInput) -> usize {
    let mut school = School::new(input);
    school.simulate(80)
}

// How many lanternfish after 256 days.
fn part2(input: AocInput) -> usize {
    let mut school = School::new(input);
    school.simulate(256)
}

// Array with fish count at each timer value.
struct School([usize; 9]);

impl School {
    fn new(input: AocInput) -> Self {
        let mut counts = [0; 9];
        for fish in input
            .single_line()
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
        {
            counts[fish] += 1;
        }

        School(counts)
    }

    fn simulate(&mut self, days: usize) -> usize {
        for _ in 0..days {
            let timer_zero_fish = self.0[0];
            for timer_val in 1..9 {
                self.0[timer_val - 1] = self.0[timer_val];
            }

            self.0[8] = timer_zero_fish; // newly created fish
            self.0[6] += timer_zero_fish; // old fish with timer reset.
        }

        self.0.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5934, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(26984457539, part2(AocInput::from_sample()));
    }
}
