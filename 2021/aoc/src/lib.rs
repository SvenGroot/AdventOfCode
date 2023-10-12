pub mod bitfield;
pub mod circular_list;
pub mod dijkstra;
pub mod grid;
pub mod grid3d;
pub mod input;
pub mod iterator;
pub mod nested_list;
pub mod tree;

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
