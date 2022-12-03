use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input(path: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(Result::unwrap)
}

pub fn get_input_vec(path: &str) -> Vec<String> {
    get_input(path).collect()
}
