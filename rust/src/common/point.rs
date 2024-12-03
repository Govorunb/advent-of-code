use crate::Size;
use std::ops::{Add, Sub};

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(tuple: (isize, isize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0 as isize,
            y: tuple.1 as isize,
        }
    }
}

impl From<Size> for Point {
    fn from(size: Size) -> Self {
        Self {
            x: size.width as isize,
            y: size.height as isize,
        }
    }
}

impl From<Point> for (isize, isize) {
    fn from(pt: Point) -> Self {
        (pt.x, pt.y)
    }
}

impl TryFrom<Point> for Size {
    type Error = ();
    fn try_from(pt: Point) -> Result<Self, Self::Error> {
        if pt.x < 0 || pt.y < 0 {
            Err(())
        } else {
            Ok(Self {
                width: pt.x as usize,
                height: pt.y as usize,
            })
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Size> for Point {
    type Output = Point;

    fn add(self, rhs: Size) -> Self::Output {
        Self {
            x: self.x + rhs.width as isize,
            y: self.y + rhs.height as isize,
        }
    }
}
