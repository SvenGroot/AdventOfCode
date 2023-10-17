pub struct SliceCombinations<'a, T> {
    slice: &'a [T],
    first: usize,
    second: usize,
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
            self.second = 0;
        }

        Some(result)
    }
}

pub trait SliceCombinator<T> {
    fn combinations(&self) -> SliceCombinations<'_, T>;
}

impl<T> SliceCombinator<T> for &[T] {
    /// Returns all combinations of two distinct items in a slice.
    ///
    /// Does not return (a, a), and will return both (a, b) and (b, a).
    fn combinations(&self) -> SliceCombinations<'_, T> {
        SliceCombinations {
            slice: self,
            first: 0,
            second: 1,
        }
    }
}
