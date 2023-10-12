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

fn part2(input: AocInput) -> usize {
    input.map(|_| 0).sum()
}

struct School(Vec<usize>);

impl School {
    fn new(input: AocInput) -> Self {
        let fish = input
            .single_line()
            .split(',')
            .map(|val| val.parse().unwrap())
            .collect();

        School(fish)
    }

    fn simulate(&mut self, days: usize) -> usize {
        for _ in 0..days {
            let mut new_fish = 0;
            for fish in &mut self.0 {
                if *fish == 0 {
                    new_fish += 1;
                    *fish = 6;
                } else {
                    *fish -= 1;
                }
            }

            self.0.resize(self.0.len() + new_fish, 8);
        }

        self.0.len()
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
        assert_eq!(0, part2(AocInput::from_sample()));
    }
}
