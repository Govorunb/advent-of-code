use std::fmt::Display;
use crate::{Direction, Direction8, Size};
use std::ops::{Add, Mul, MulAssign, Neg, Sub};

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
    pub fn adjacent(self) -> impl Iterator<Item = Vector2> {
        Direction::deltas()
            .map(move |&offset| self + offset)
    }
    pub fn around(self) -> impl Iterator<Item = Vector2> {
        Direction8::deltas()
            .map(move |&offset| self + offset)
    }
    
    /// Returns an iterator that moves the given `step` each iteration.<br/>
    /// Note: this iterator **does not terminate**.
    pub fn ray(self, step: Vector2) -> impl Iterator<Item = Vector2> {
        let mut curr = self;
        std::iter::from_fn(move || {
            let item = curr;
            curr += step;
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
impl From<Direction> for Vector2 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => Vector2 {x: 0, y:-1},
            Direction::South => Vector2 {x: 0, y: 1},
            Direction::East  => Vector2 {x: 1, y: 0},
            Direction::West  => Vector2 {x:-1, y: 0},
        }
    }
}

impl From<Direction8> for Vector2 {
    fn from(dir: Direction8) -> Self {
        match dir {
            Direction8::North => Vector2 {x: 0, y: -1},
            Direction8::South => Vector2 {x: 0, y:  1},
            Direction8::West  => Vector2 {x:-1, y:  0},
            Direction8::East  => Vector2 {x: 1, y:  0},
            Direction8::NorthWest => Vector2 {x:-1, y: -1},
            Direction8::NorthEast => Vector2 {x: 1, y: -1},
            Direction8::SouthWest => Vector2 {x: 0, y:  1},
            Direction8::SouthEast => Vector2 {x: 1, y:  0},
        }
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

impl Add for &Vector2 { type Output = Vector2; fn add(self, rhs: Self) -> Self::Output { *self + *rhs } }
impl Sub for &Vector2 { type Output = Vector2; fn sub(self, rhs: Self) -> Self::Output { *self - *rhs } }
impl Neg for &Vector2 { type Output = Vector2; fn neg(self) -> Self::Output { -*self } }

impl Add<Size> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Size) -> Self::Output {
        Self {
            x: self.x + rhs.width as isize,
            y: self.y + rhs.height as isize,
        }
    }
}

impl Add<Direction> for Vector2 { type Output = Self; fn add(self, rhs: Direction) -> Self::Output { self + Self::from(rhs) } }
impl Sub<Direction> for Vector2 { type Output = Self; fn sub(self, rhs: Direction) -> Self::Output { self - Self::from(rhs) } }
impl Add<Direction8> for Vector2 { type Output = Self; fn add(self, rhs: Direction8) -> Self::Output { self + Self::from(rhs) } }
impl Sub<Direction8> for Vector2 { type Output = Self; fn sub(self, rhs: Direction8) -> Self::Output { self - Self::from(rhs) } }
impl std::ops::AddAssign<Direction> for Vector2 { fn add_assign(&mut self, rhs: Direction) { *self += Self::from(rhs) } }
impl std::ops::SubAssign<Direction> for Vector2 { fn sub_assign(&mut self, rhs: Direction) { *self -= Self::from(rhs) } }
impl std::ops::AddAssign<Direction8> for Vector2 { fn add_assign(&mut self, rhs: Direction8) { *self += Self::from(rhs) } }
impl std::ops::SubAssign<Direction8> for Vector2 { fn sub_assign(&mut self, rhs: Direction8) { *self -= Self::from(rhs) } }

impl Mul<isize> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
impl Mul<usize> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: usize) -> Self::Output {
        self.mul(rhs as isize)
    }
}
impl Mul<Vector2> for isize {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {rhs*self}
}
impl Mul<Vector2> for usize {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {rhs*self}
}
impl MulAssign<isize> for Vector2 {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl MulAssign<usize> for Vector2 {
    fn mul_assign(&mut self, rhs: usize) {
        self.mul_assign(rhs as isize)
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
