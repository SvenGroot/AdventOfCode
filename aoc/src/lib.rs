pub mod dijkstra;
pub mod grid;
pub mod grid3d;
pub mod iterator;
pub mod nested_list;

use std::{
    env::current_exe,
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::{Component, Path, PathBuf},
};

use num::Integer;

pub fn aoc_input() -> PathBuf {
    aoc_input_core(false)
}

pub fn aoc_sample_input() -> PathBuf {
    aoc_input_core(true)
}

fn aoc_input_core(sample: bool) -> PathBuf {
    let path = current_exe().unwrap();
    let mut name = path.file_name().unwrap().to_str().unwrap();
    if let Some(index) = name.find('-') {
        name = &name[..index];
    }

    let mut input: PathBuf = path
        .components()
        .take_while(|c| *c != Component::Normal(OsStr::new("target")))
        .collect();

    input.push("input");
    if sample {
        input.push("sample");
    }

    input.push(name);
    input.set_extension("txt");
    input
}

pub fn get_input(path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(Result::unwrap)
}

pub fn get_input_vec(path: impl AsRef<Path>) -> Vec<String> {
    get_input(path).collect()
}

pub fn get_input_single(path: impl AsRef<Path>) -> String {
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
