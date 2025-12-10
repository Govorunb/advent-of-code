pub trait MapTuple<T, F> {
    type Output;
    fn map(self, func: F) -> Self::Output;
}

impl<T, U, F> MapTuple<T, F> for (T, T)
where
F: Fn(T) -> U
{
    type Output = (U, U);
    #[inline(always)]
    fn map(self, func: F) -> Self::Output {
        (func(self.0), func(self.1))
    }
}

impl<T, U, F> MapTuple<T, F> for &(T, T)
where
F: Fn(&T) -> U
{
    type Output = (U, U);
    #[inline(always)]
    fn map(self, func: F) -> Self::Output {
        (func(&self.0), func(&self.1))
    }
}

impl<T, U, F> MapTuple<T, F> for (T, T, T)
where
F: Fn(T) -> U
{
    type Output = (U, U, U);
    #[inline(always)]
    fn map(self, func: F) -> Self::Output {
        (func(self.0), func(self.1), func(self.2))
    }
}

impl<T, U, F> MapTuple<T, F> for (T, T, T, T)
where
F: Fn(T) -> U
{
    type Output = (U, U, U, U);
    #[inline(always)]
    fn map(self, func: F) -> Self::Output {
        (func(self.0), func(self.1), func(self.2), func(self.3))
    }
}


pub trait ParseTuple<T> {
    type Output;
    fn parse(self) -> Self::Output;
}


pub trait SortTuple<T: PartialOrd> {
    type Output;
    fn sort(self) -> Self::Output;
}

impl<T: PartialOrd> SortTuple<T> for (T, T) {
    type Output = (T, T);

    fn sort(self) -> Self::Output {
        if self.0 <= self.1 {
            self
        } else {
            (self.1, self.0)
        }
    }
}

impl<T: Ord> SortTuple<T> for (T, T, T) {
    type Output = (T, T, T);

    fn sort(self) -> Self::Output {
        let mut slice = [self.0, self.1, self.2];
        slice.sort();
        let [a,b,c] = slice;
        (a,b,c)
    }
}
