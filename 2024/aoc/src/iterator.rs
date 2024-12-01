use std::iter::Peekable;

pub struct InfiniteRepeat<I: Iterator + Clone> {
    source: I,
    current: I,
}

impl<I: Iterator + Clone> Iterator for InfiniteRepeat<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.current.next();
        if result.is_none() {
            self.current = self.source.clone();
            result = self.current.next();
        }

        result
    }
}

pub trait InfiniteRepeatExt
where
    Self: Iterator + Clone,
{
    fn infinite_repeat(self) -> InfiniteRepeat<Self> {
        InfiniteRepeat {
            source: self.clone(),
            current: self,
        }
    }
}

impl<T: Iterator + Clone> InfiniteRepeatExt for T {}

pub struct TakeWhilePeek<'a, I: Iterator, P> {
    peekable: &'a mut Peekable<I>,
    predicate: P,
    done: bool,
}

impl<I: Iterator, P: FnMut(&I::Item) -> bool> Iterator for TakeWhilePeek<'_, I, P> {
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

pub trait IteratorExt
where
    Self: Iterator + Sized,
{
    fn into_vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}

impl<T: Iterator> IteratorExt for T {}
