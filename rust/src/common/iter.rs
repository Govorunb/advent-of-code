use std::iter::FusedIterator;

pub struct SlidingPairs<I: Iterator> {
    iter: I,
    last: Option<I::Item>,
}

pub trait SlidingPairsExt: Iterator
{
    fn sliding_pairs(self) -> SlidingPairs<Self> where Self: std::marker::Sized;
}


impl<I: Iterator> SlidingPairsExt for I
{
    fn sliding_pairs(self) -> SlidingPairs<Self> {
        SlidingPairs { iter: self, last: None }
    }
}

impl<I: Iterator> Iterator for SlidingPairs<I> 
where I::Item : Clone {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.last.is_none() {
            self.last = self.iter.next();
            self.last.as_ref()?;
        }

        let next = self.iter.next();
        next.as_ref()?;

        let item = (self.last.take().unwrap(), next.clone().unwrap());
        self.last = next;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let inner = self.iter.size_hint();
        (inner.0.saturating_sub(1), inner.1)
    }
}

impl<I: FusedIterator> FusedIterator for SlidingPairs<I>
    where I::Item : Clone
{}

impl<I: ExactSizeIterator> ExactSizeIterator for SlidingPairs<I>
    where I::Item : Clone
{
    fn len(&self) -> usize {
        self.iter.len().saturating_sub(1)
    }
}