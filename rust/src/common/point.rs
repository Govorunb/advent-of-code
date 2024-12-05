use crate::{Direction, Direction8, Size};
use std::ops::{Add, Sub};

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Default)]
#[derive(derive_more::Add, derive_more::Sub, derive_more::From)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn adjacent<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        Direction::iter()
            .map(move |offset| self + offset)
    }
    pub fn around<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        Direction8::iter()
            .map(move |offset| self + offset)
    }
    
    /// Returns an iterator that moves in the given direction.<br/>
    /// Note: this iterator **does not terminate**.
    pub fn ray<'a>(&self, dir: &'a Point) -> impl Iterator<Item = Point> + 'a {
        let mut curr = self.clone();
        std::iter::from_fn(move || {
            let item = curr.clone();
            curr = &curr + dir;
            Some(item)
        })
    }
}

impl From<Size> for Point {
    fn from(size: Size) -> Self {
        Self::from((size.width, size.height))
    }
}

impl From<(usize, usize)> for Point {
    fn from((x,y): (usize, usize)) -> Self {
        Self { x: x as isize, y: y as isize }
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

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;
    
    fn add(self, rhs: Self) -> Self::Output {
        Point {
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
