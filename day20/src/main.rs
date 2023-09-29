// https://adventofcode.com/2022/day/20
use std::path::Path;

use aoc::{aoc_input, circular_list::CircularList, get_input};

fn main() {
    let path = aoc_input();
    println!("Part 1: {}", part1(&path));
    println!("Part 2: {}", part2(&path));
}

fn part1(path: impl AsRef<Path>) -> i32 {
    let data: Vec<i32> = get_input(path).map(|line| line.parse().unwrap()).collect();
    let mut places = CircularList::new((0..data.len()).collect());
    for (index, &value) in data.iter().enumerate() {
        let current_place = places
            .as_slice()
            .iter()
            .position(|&place| place == index)
            .unwrap();

        places.move_item(current_place, value as isize);
    }

    // Index of item '0' in data.
    let index = data.iter().position(|&item| item == 0).unwrap();
    // Place of that item
    let place = places.as_slice().iter().position(|&i| i == index).unwrap();
    let value1 = data[places[place + 1000]];
    let value2 = data[places[place + 2000]];
    let value3 = data[places[place + 3000]];
    println!("Values are {value1}, {value2}, {value3}");
    value1 + value2 + value3
}

fn part2(path: impl AsRef<Path>) -> isize {
    let key = 811589153;
    let data: Vec<_> = get_input(path)
        .map(|line| line.parse::<isize>().unwrap() * key)
        .collect();

    let mut places = CircularList::new((0..data.len()).collect());
    for _ in 0..10 {
        for (index, &value) in data.iter().enumerate() {
            let current_place = places
                .as_slice()
                .iter()
                .position(|&place| place == index)
                .unwrap();

            places.move_item(current_place, value);
        }
    }

    // Index of item '0' in data.
    let index = data.iter().position(|&item| item == 0).unwrap();
    // Place of that item
    let place = places.as_slice().iter().position(|&i| i == index).unwrap();
    let value1 = data[places[place + 1000]];
    let value2 = data[places[place + 2000]];
    let value3 = data[places[place + 3000]];
    println!("Values are {value1}, {value2}, {value3}");
    value1 + value2 + value3
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1623178306, part2(aoc_sample_input()));
    }
}
