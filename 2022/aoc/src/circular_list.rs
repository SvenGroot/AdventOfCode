use std::ops::{Index, IndexMut};

pub struct CircularList<T>(Vec<T>);

impl<T> CircularList<T> {
    pub fn new(source: Vec<T>) -> Self {
        Self(source)
    }

    pub fn move_item(&mut self, index: usize, amount: isize) {
        // Last and first item are considered adjacent, so moving the last item +1 moves it to
        // index 1, not 0, hence the modulo len - 1.
        let new_index = ((index as isize + amount).rem_euclid((self.0.len() - 1) as isize))
            .try_into()
            .unwrap();

        if new_index == index {
            return;
        }

        let value = self.0.remove(index);
        self.0.insert(new_index, value);
    }

    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> Index<usize> for CircularList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index % self.0.len()]
    }
}

impl<T> IndexMut<usize> for CircularList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.0.len();
        &mut self.0[index % len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let mut list = CircularList::new(vec![4, 5, 6, 1, 7, 8, 9]);
        list.move_item(3, 2);
        assert_eq!(&[4, 5, 6, 7, 8, 1, 9], list.as_slice());

        list.move_item(3, -2);
        assert_eq!(&[4, 7, 5, 6, 8, 1, 9], list.as_slice());

        list.move_item(1, -2);
        assert_eq!(&[4, 5, 6, 8, 1, 7, 9], list.as_slice());

        list.move_item(6, 1);
        assert_eq!(&[4, 9, 5, 6, 8, 1, 7], list.as_slice());

        list.move_item(3, 20);
        assert_eq!(&[4, 9, 5, 8, 1, 6, 7], list.as_slice());

        list.move_item(5, -20);
        assert_eq!(&[4, 9, 5, 6, 8, 1, 7], list.as_slice());
    }
}
