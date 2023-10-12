use std::{
    env::current_exe,
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Map,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use crate::iterator::IntoVec;

pub type FileInput = Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String>;

pub struct AocInput(FileInput);

impl AocInput {
    pub fn get_path(sample: bool) -> PathBuf {
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

    pub fn from_file(path: impl AsRef<Path>) -> Self {
        AocInput(
            BufReader::new(File::open(path).unwrap())
                .lines()
                .map(Result::unwrap),
        )
    }

    pub fn from_input() -> Self {
        Self::from_file(Self::get_path(false))
    }

    pub fn from_sample() -> Self {
        Self::from_file(Self::get_path(true))
    }

    pub fn single_line(mut self) -> String {
        self.0.next().unwrap()
    }

    pub fn single_line_parsed<U: FromStr>(self, separator: char) -> Vec<U>
    where
        <U as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.single_line()
            .split(separator)
            .map(|val| val.parse().unwrap())
            .collect()
    }

    pub fn into_vec(self) -> Vec<String> {
        self.0.into_vec()
    }
}

impl AocInput {
    /// Parses the input as the specified type, panicking if anything is invalid.
    pub fn parsed<T: FromStr>(self) -> impl Iterator<Item = T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.0.map(|line| T::from_str(&line).unwrap())
    }

    /// Parses the input as the specified type, panicking if anything is invalid, but blank lines
    /// return `None`.
    pub fn parsed_opt<T: FromStr>(self) -> impl Iterator<Item = Option<T>>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.0
            .map(|line| (!line.is_empty()).then(|| T::from_str(&line).unwrap()))
    }
}

impl Iterator for AocInput {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
