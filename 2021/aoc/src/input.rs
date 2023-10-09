use std::{path::Path, str::FromStr};

use crate::{aoc_input, aoc_sample_input, get_input, iterator::IntoVec, FileInput};

#[derive(Clone)]
pub struct AocInput<T = String, I = FileInput>(I)
where
    I: Iterator<Item = T>;

impl AocInput<String, FileInput> {
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        AocInput(get_input(path))
    }

    pub fn from_input() -> Self {
        Self::from_file(aoc_input())
    }

    pub fn from_sample() -> Self {
        Self::from_file(aoc_sample_input())
    }
}

impl<I: Iterator<Item = String>> AocInput<String, I> {
    /// Parses the input as the specified type, panicking if anything is invalid.
    pub fn parsed<T: FromStr>(self) -> AocInput<T, impl Iterator<Item = T>>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        AocInput(self.0.map(|line| T::from_str(&line).unwrap()))
    }

    /// Parses the input as the specified type, panicking if anything is invalid, but blank lines
    /// return `None`.
    pub fn parsed_opt<T: FromStr>(self) -> AocInput<Option<T>, impl Iterator<Item = Option<T>>>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        AocInput(
            self.0
                .map(|line| (!line.is_empty()).then(|| T::from_str(&line).unwrap())),
        )
    }
}

impl<T, I: Iterator<Item = T>> AocInput<T, I> {
    pub fn into_vec(self) -> Vec<T> {
        self.0.into_vec()
    }
}

impl<T, I: Iterator<Item = T>> Iterator for AocInput<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
