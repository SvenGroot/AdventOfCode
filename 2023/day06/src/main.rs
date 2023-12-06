// https://adventofcode.com/2023/day/6

use aoc::input::AocInput;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Multiply the number of ways you can beat the record for each race.
fn part1(input: AocInput) -> usize {
    let races = Races::from_input(input);
    races.0.iter().map(Race::winning_times_count).product()
}

// Treat the input as one race by combining the numbers, ignoring spaces.
fn part2(input: AocInput) -> usize {
    let race = Race::from_input(input);
    race.winning_times_count()
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn from_input(mut input: AocInput) -> Self {
        Self {
            time: Self::parse_line(&input.next().unwrap()),
            distance: Self::parse_line(&input.next().unwrap()),
        }
    }

    fn winning_times_count(&self) -> usize {
        // We need to find the lowest and highest button push that beats the record. This means
        // solving: (time - push_time) * push_time >= distance
        // This is equal to: -push_time^2 + time * push_time - distance >= 0
        // We can solve for push_time with == 0 using the quadratic formula.
        let time = self.time as f64;
        let distance = self.distance as f64;
        let sqrt = (time * time - (-4.0 * -distance)).sqrt();
        // Using floor + 1 and ceil - 1 handles the case where the solution is an integer, which
        // would equal but not exceed the record.
        let min = ((-time + sqrt) / -2.0).floor() as usize + 1;
        let max = ((-time - sqrt) / -2.0).ceil() as usize - 1;
        max - min + 1
    }

    fn parse_line(line: &str) -> usize {
        line.split(' ')
            .filter(|part| !part.is_empty())
            .skip(1)
            .fold(String::new(), |s, part| s + part)
            .parse()
            .unwrap()
    }
}

struct Races(Vec<Race>);

impl Races {
    fn from_input(mut input: AocInput) -> Self {
        let times = input.next().unwrap();
        let distances = input.next().unwrap();
        let times = Self::parse_line(&times);
        let distances = Self::parse_line(&distances);
        Self(
            times
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect(),
        )
    }

    fn parse_line(line: &str) -> impl Iterator<Item = usize> + '_ {
        line.split(' ')
            .filter(|part| !part.is_empty())
            .skip(1)
            .map(|value| value.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(288, part1(AocInput::from_sample()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(71503, part2(AocInput::from_sample()));
    }
}
