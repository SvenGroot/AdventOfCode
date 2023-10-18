use std::{collections::HashSet, hash::Hash};

pub struct SliceCombinations<'a, T> {
    slice: &'a [T],
    first: usize,
    second: usize,
    ordered: bool,
}

impl<'a, T> SliceCombinations<'a, T> {
    fn new(slice: &'a [T], ordered: bool) -> Self {
        Self {
            slice,
            first: 0,
            second: 1,
            ordered,
        }
    }
}

impl<'a, T> Iterator for SliceCombinations<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first >= self.slice.len() || self.second >= self.slice.len() {
            return None;
        }

        let result = (&self.slice[self.first], &self.slice[self.second]);
        self.second += 1;
        if self.second == self.first {
            self.second += 1;
        }

        if self.second == self.slice.len() {
            self.first += 1;
            self.second = if self.ordered { 0 } else { self.first + 1 };
        }

        Some(result)
    }
}

pub trait SliceExt<T> {
    /// Returns all combinations of two distinct items in a slice.
    ///
    /// Does not return (a, a), and will return both (a, b) and (b, a).
    fn combinations(&self) -> SliceCombinations<'_, T>;

    /// Returns all combinations of two distinct items in a slice, when the order of items in the
    /// pair does not matter.
    ///
    /// Does not return (a, a), and if (a, b) is returned, (b, a) will not be.
    fn unordered_combinations(&self) -> SliceCombinations<'_, T>;

    fn get_two_mut(&mut self, index1: usize, index2: usize) -> (&mut T, &mut T);
}

impl<T> SliceExt<T> for [T] {
    fn combinations(&self) -> SliceCombinations<'_, T> {
        SliceCombinations::new(self, true)
    }

    fn unordered_combinations(&self) -> SliceCombinations<'_, T> {
        SliceCombinations::new(self, false)
    }

    fn get_two_mut(&mut self, index1: usize, index2: usize) -> (&mut T, &mut T) {
        let lower = index1.min(index2);
        let higher = index1.max(index2);
        let split = lower + 1;
        let (first, second) = self.split_at_mut(split);
        let (first, second) = (&mut first[lower], &mut second[higher - split]);
        if index1 < index2 {
            (first, second)
        } else {
            (second, first)
        }
    }
}

pub trait Intersect<T>
where
    Self: ToOwned,
{
    fn intersect(&self, other: Self) -> Vec<&T>;
}

impl<T: Eq + Hash> Intersect<T> for &[T] {
    fn intersect(&self, other: Self) -> Vec<&T> {
        let items: HashSet<_> = self.iter().collect();
        other.iter().filter(|item| items.contains(item)).collect()
    }
}
