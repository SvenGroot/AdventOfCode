pub struct InfiniteRepeat<'a, T> {
    items: &'a [T],
    index: usize,
}

impl<'a, T> Iterator for InfiniteRepeat<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.items.get(self.index)?;
        self.index = (self.index + 1) % self.items.len();
        Some(result)
    }
}

impl<'a, T> InfiniteRepeat<'a, T> {
    pub fn index(&self) -> usize {
        self.index
    }
}

pub fn infinite_repeat<T>(items: &[T]) -> InfiniteRepeat<T> {
    InfiniteRepeat { items, index: 0 }
}
