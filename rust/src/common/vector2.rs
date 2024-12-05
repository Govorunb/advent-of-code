use crate::{Direction, Direction8, Size};
use std::ops::{Add, Neg, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Default)]
#[derive(derive_more::Add, derive_more::AddAssign, derive_more::Sub, derive_more::SubAssign, derive_more::From, derive_more::Neg)]
pub struct Vector2 {
    pub x: isize,
    pub y: isize,
}

impl Vector2 {
    pub fn zero() -> Vector2 {Self::default()}
    pub fn one() -> Vector2 { Self { x: 1, y: 1 } }
    pub fn up() -> Vector2 { Self { x: 0, y: -1 } }
    pub fn down() -> Vector2 { Self { x: 0, y: 1 } }
    pub fn left() -> Vector2 { Self { x: -1, y: 0 } }
    pub fn right() -> Vector2 { Self { x: 1, y: 0 } }
    pub fn adjacent<'a>(&'a self) -> impl Iterator<Item = Vector2> + 'a {
        Direction::deltas()
            .map(move |offset| self + offset)
    }
    pub fn around<'a>(&'a self) -> impl Iterator<Item = Vector2> + 'a {
        Direction8::deltas()
            .map(move |offset| self + offset)
    }
    
    /// Returns an iterator that moves in the given direction each step.<br/>
    /// Note: this iterator **does not terminate**.
    pub fn ray<'a>(&self, dir: &'a Vector2) -> impl Iterator<Item = Vector2> + 'a {
        let mut curr = self.clone();
        std::iter::from_fn(move || {
            let item = curr.clone();
            curr = &curr + dir;
            Some(item)
        })
    }
}

impl From<Size> for Vector2 {
    fn from(size: Size) -> Self {
        Self::from((size.width, size.height))
    }
}

impl From<(usize, usize)> for Vector2 {
    fn from((x,y): (usize, usize)) -> Self {
        Self { x: x as isize, y: y as isize }
    }
}

impl From<Vector2> for (isize, isize) {
    fn from(pt: Vector2) -> Self {
        (pt.x, pt.y)
    }
}

impl TryFrom<Vector2> for Size {
    type Error = ();
    fn try_from(pt: Vector2) -> Result<Self, Self::Error> {
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

impl Sub for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for &Vector2 {
    type Output = Vector2;
    
    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Size> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Size) -> Self::Output {
        Self {
            x: self.x + rhs.width as isize,
            y: self.y + rhs.height as isize,
        }
    }
}

impl Neg for &Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output {
        Vector2 {x: -self.x, y: -self.y}
    }
}
