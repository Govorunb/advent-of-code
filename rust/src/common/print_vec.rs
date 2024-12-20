use std::fmt;
use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub struct PrintVec<T>(pub Vec<T>);

impl<T: Display> Display for PrintVec<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.iter().join(", "))
    }
}
