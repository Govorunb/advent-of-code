use itertools::{Either, Itertools};

pub trait Pick<T> {
    fn pick(self, indices: &[usize]) -> impl Iterator<Item = T>;
}

impl<T, I: Iterator<Item = T>> Pick<T> for &mut I {
    /// Picks out items at the specified `indices` from the underlying `iterator`.
    /// 
    /// Elements are returned in `iterator` order regardless of the `indices` order; larger `indices` will perform better if sorted.
    /// 
    /// Do not include duplicates in `indices` - sorted will break and unsorted will only return one.
    fn pick(self, indices: &[usize]) -> impl Iterator<Item = T> {
        if indices.is_empty() {
            return Either::Left(std::iter::empty());
        }
        let is_sorted = indices.is_sorted();
        
        if indices.iter().duplicates().next().is_some() {
            todo!("duplicate in indices")
        }
        let mut count = 0;
        let mut iter = self.enumerate();
        Either::Right(std::iter::from_fn(move || {
            while let Some((i, item)) = iter.next() {
                // already returned all requested
                if count >= indices.len() {return None}
                
                // we can rely on the sort to be slightly more efficient
                if is_sorted && i == indices[count]
                || !is_sorted && indices.contains(&i) {
                    count += 1;
                    return Some(item)
                }
            }
            None
        }).fuse())
    }
}

#[test]
fn test_pick() {
    let items = [1,2,3,4,5,6,7,8,9,10];
    let indices = [0,3,5,999];

    let mut iter = items.into_iter();
    let picked = iter.pick(&indices);
    assert!(picked.eq([1,4,6]));
}

pub trait Rle<T> {
    /// Consumes the given iterator and returns [run-length](https://en.wikipedia.org/wiki/Run-length_encoding) tuples.
    fn rle(self) -> impl Iterator<Item = (T, usize)>;
}

impl<T: PartialEq + Clone, I: Iterator<Item = T> + Clone> Rle<T> for I {
    fn rle(self) -> impl Iterator<Item=(T, usize)> {
        // adapted from https://stackoverflow.com/a/55676567 (which is absolute magic)
        self.peekable()
            .batching(|it| {
                it.next()
                    .map(|v| (v.clone(), 1 + it.peeking_take_while(|v2| *v2 == v).count()))
                    
            })
    }
}
