// https://adventofcode.com/2022/day/1
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Result;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            result.push(sum);
            sum = 0;
        } else {
            sum += u32::from_str(&line)?;
        }
    }

    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    println!("Sum: {}", sum);

    Ok(())
}
