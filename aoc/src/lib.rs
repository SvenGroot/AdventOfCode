pub mod dijkstra;
pub mod sliding_window;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use num::Integer;

pub fn get_input(path: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(Result::unwrap)
}

pub fn get_input_vec(path: &str) -> Vec<String> {
    get_input(path).collect()
}

pub fn get_input_single(path: &str) -> String {
    let mut line = String::new();
    BufReader::new(File::open(path).unwrap())
        .read_line(&mut line)
        .unwrap();

    line
}

pub trait Lcm<T> {
    fn lcm(&mut self) -> Option<T>;
}

impl<T, U> Lcm<T> for U
where
    T: Integer + Copy,
    U: Iterator<Item = T>,
{
    fn lcm(&mut self) -> Option<T> {
        let mut lcm = self.next()?;
        for val in self {
            lcm = num::integer::lcm(lcm, val);
        }

        Some(lcm)
    }
}
