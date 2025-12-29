pub mod bitfield;
pub mod bitreader;
pub mod circular_list;
pub mod dijkstra;
pub mod graph;
pub mod grid;
pub mod grid3d;
pub mod input;
pub mod iterator;
pub mod nested_list;
pub mod slice;
pub mod tree;

use std::collections::HashMap;

use num::Integer;

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

#[derive(Default)]
pub struct NameMap {
    map: HashMap<String, usize>,
    next: usize,
}

impl NameMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn map(&mut self, name: String) -> usize {
        *self.map.entry(name).or_insert_with(|| {
            let result = self.next;
            self.next += 1;
            result
        })
    }
}
