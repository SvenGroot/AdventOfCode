use std::iter::Peekable;

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

pub struct TakeWhilePeek<'a, I: Iterator, P> {
    peekable: &'a mut Peekable<I>,
    predicate: P,
    done: bool,
}

impl<'a, I: Iterator, P: FnMut(&I::Item) -> bool> Iterator for TakeWhilePeek<'a, I, P> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let x = self.peekable.peek()?;
        if (self.predicate)(x) {
            self.peekable.next()
        } else {
            self.done = true;
            None
        }
    }
}

pub trait PeekableExt<I: Iterator> {
    fn take_while_peek<P>(&mut self, predicate: P) -> TakeWhilePeek<I, P>
    where
        Self: Sized,
        P: FnMut(&I::Item) -> bool;
}

impl<I: Iterator> PeekableExt<I> for Peekable<I> {
    fn take_while_peek<P>(&mut self, predicate: P) -> TakeWhilePeek<I, P>
    where
        Self: Sized,
        P: FnMut(&<I as Iterator>::Item) -> bool,
    {
        TakeWhilePeek {
            peekable: self,
            predicate,
            done: false,
        }
    }
}

pub trait IntoVec
where
    Self: Iterator + Sized,
{
    fn into_vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}

impl<T: Iterator> IntoVec for T {}
