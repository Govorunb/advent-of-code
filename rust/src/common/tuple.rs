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
