pub struct SlidingWindow<'a, T> {
    storage: &'a [T],
    index: usize,
    window_size: usize,
}

impl<'a, T> SlidingWindow<'a, T> {
    pub fn new(storage: &'a [T], window_size: usize) -> Self {
        Self {
            storage,
            index: 0,
            window_size,
        }
    }
}

impl<'a, T> Iterator for SlidingWindow<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.window_size <= self.storage.len() {
            let result = Some(&self.storage[self.index..self.index + self.window_size]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

pub trait HasSlidingWindow<T> {
    fn sliding_window(&self, window_size: usize) -> SlidingWindow<T>;
}

impl<T> HasSlidingWindow<T> for &[T] {
    fn sliding_window(&self, window_size: usize) -> SlidingWindow<T> {
        SlidingWindow::new(self, window_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let items = [1, 2, 3, 4, 5, 6, 7];
        let mut window = SlidingWindow::new(&items, 3);
        assert_eq!(Some([1, 2, 3].as_ref()), window.next());
        assert_eq!(Some([2, 3, 4].as_ref()), window.next());
        assert_eq!(Some([3, 4, 5].as_ref()), window.next());
        assert_eq!(Some([4, 5, 6].as_ref()), window.next());
        assert_eq!(Some([5, 6, 7].as_ref()), window.next());
        assert!(window.next().is_none())
    }
}
