// https://adventofcode.com/2022/day/1
use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

use anyhow::Result;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result = reader
        .lines()
        .map(|l| l)
        .collect::<std::io::Result<Vec<_>>>()?
        .split(|l| l.len() == 0)
        .map(|split| split.iter().map(|l| u32::from_str(l)).sum())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    println!("Sum: {}", sum);

    Ok(())
}
