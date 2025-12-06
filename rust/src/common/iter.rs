use itertools::{Either, Itertools};

pub trait Pick: Iterator {
    fn pick(self, indices: &[usize]) -> impl Iterator<Item = Self::Item>;
}

impl<I: Iterator> Pick for &mut I {
    /// Picks out items at the specified `indices` from the underlying `iterator`.
    /// 
    /// Elements are returned in `iterator` order regardless of the `indices` order; larger `indices` will perform better if sorted.
    /// 
    /// Do not include duplicates in `indices` - sorted will break and unsorted will only return one.
    fn pick(self, indices: &[usize]) -> impl Iterator<Item = Self::Item> {
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
            for (i, item) in iter.by_ref() {
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

pub trait Rle: Iterator {
    /// Consumes the given iterator and returns [run-length](https://en.wikipedia.org/wiki/Run-length_encoding) tuples.
    fn rle(self) -> impl Iterator<Item = (Self::Item, usize)>;
}

pub trait RleBy: Iterator {
    fn rle_by<K, F>(self, key_selector: F) -> impl Iterator<Item = (Self::Item, usize)>
        where
            F: Fn(&Self::Item) -> K,
            K: PartialEq;
}

impl<I> Rle for I
where
    I: Iterator + Clone,
    Self::Item: PartialEq + Clone,
{
    fn rle(self) -> impl Iterator<Item=(Self::Item, usize)> {
        // adapted from https://stackoverflow.com/a/55676567 (which is absolute magic)
        self.peekable()
            .batching(|it| {
                it.next()
                    .map(|v| (v.clone(), 1 + it.peeking_take_while(|v2| *v2 == v).count()))
                    
            })
    }
}

impl<I: Iterator> RleBy for I {
    fn rle_by<K, F>(self, key_selector: F) -> impl Iterator<Item=(Self::Item, usize)>
    where
        F: Fn(&Self::Item) -> K,
        K: PartialEq
    {
        self.peekable()
            .batching(move |it| {
                it.next()
                    .map(|v| {
                        let key = key_selector(&v);
                        (v, 1 + it.peeking_take_while(|v2| key_selector(v2) == key).count())
                    })
            })
    }
}

pub trait TriangleProduct: Iterator + Clone {
    /// Used on a sorted iterator, this produces pairs where the second item is at least as large (in the iterator's sort order) than the first.
    /// This sort of "triangle product" runs in about half as much time as a normal cartesian product (specifically, `(n(n+1))/2` rather than `n^2`).
    /// Not very useful if called on an unsorted iterator, but you do you.
    fn triangle_product(self) -> impl Iterator<Item = (Self::Item, Self::Item)>;
}

impl<I> TriangleProduct for I
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn triangle_product(self) -> impl Iterator<Item = (Self::Item, Self::Item)> {
        let copy = self.clone();
        self.enumerate()
            .flat_map(move |(i, e)| {
                copy.clone()
                    .skip(i)
                    .map(move |c| (e.clone(), c))
            })
    }
}

pub fn transpose<I, U>(source: I) -> impl Iterator<Item = impl Iterator<Item = U::Item>>
where
    I: Iterator<Item = U>,
    U: Iterator
{
    let mut iters = source.collect_vec();
    std::iter::from_fn(move || {
        let mut v = vec![];
        for u in &mut iters {
            let Some(item) = u.next()
                else {return None};
            v.push(item);
        }
        Some(v.into_iter())
    })
}
