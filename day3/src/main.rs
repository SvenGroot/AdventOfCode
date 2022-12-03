// https://adventofcode.com/2022/day/3
// I've decided to make things easier on myself and just unwrap stuff instead of dealing with errors
// properly.
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("input/day3.txt").unwrap();
    let reader = BufReader::new(file);
    let sum: u32 = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let left = &line[0..line.len() / 2];
            let right = &line[line.len() / 2..];
            let common = left.chars().find(|ch| right.contains(*ch)).unwrap();
            get_priority(common)
        })
        .sum();

    println!("Sum: {}", sum);
}

fn part2() {
    let file = File::open("input/day3.txt").unwrap();
    let reader = BufReader::new(file);
    let sum: u32 = reader
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .chunks(3) // array_chunks could do this without the collect but it's unstable
        .map(|group| {
            let common = group[0]
                .chars()
                .find(|ch| group[1..].iter().all(|g| g.contains(*ch)))
                .unwrap();
            get_priority(common)
        })
        .sum();

    println!("Sum: {}", sum);
}

fn get_priority(ch: char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => panic!("Invalid input."),
    }
}
