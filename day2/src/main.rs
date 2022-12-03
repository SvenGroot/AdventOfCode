// https://adventofcode.com/2022/day/2
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);
    let score: Result<i32> = reader
        .lines()
        .map(|line| {
            let line = line?;
            let opponent_choice = line.as_bytes()[0] as i32 - 'A' as i32;
            let outcome = line.as_bytes()[2] as i32 - 'Y' as i32;
            let my_choice = (opponent_choice + outcome).rem_euclid(3);
            let points = match outcome {
                1 => 6,
                0 => 3,
                _ => 0,
            } + my_choice
                + 1;
            Ok(points)
        })
        .sum();

    println!("Total score: {}", score?);
    Ok(())
}
