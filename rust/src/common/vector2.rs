use std::fmt::Display;
use crate::{Direction, Direction8, Size};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Default)]
pub struct Vector2 {
    pub x: isize,
    pub y: isize,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0, y: 0 };
    pub const ONE: Vector2 = Vector2 { x: 1, y: 1 };
    pub const UP: Vector2 = Vector2 { x: 0, y: -1 };
    pub const DOWN: Vector2 = Vector2 { x: 0, y: 1 };
    pub const RIGHT: Vector2 = Vector2 { x: 1, y: 0 };
    pub const LEFT: Vector2 = Vector2 { x: -1, y: 0 };
    pub const TOP_LEFT: Vector2 = Vector2::UP + Vector2::LEFT;
    pub const TOP_RIGHT: Vector2 = Vector2::UP + Vector2::RIGHT;
    pub const BOTTOM_LEFT: Vector2 = Vector2::DOWN + Vector2::LEFT;
    pub const BOTTOM_RIGHT: Vector2 = Vector2::DOWN + Vector2::RIGHT;
    pub const AROUND: [Vector2; 8] = [
        Vector2::TOP_LEFT,    Vector2::UP,   Vector2::TOP_RIGHT,
        Vector2::LEFT,                       Vector2::RIGHT,
        Vector2::BOTTOM_LEFT, Vector2::DOWN, Vector2::BOTTOM_RIGHT,
    ];
    pub const ADJACENT: [Vector2; 4] = [
                              Vector2::UP,
        Vector2::LEFT,                       Vector2::RIGHT,
                              Vector2::DOWN,
    ];
    pub const CORNERS: [Vector2; 4] = [
        Vector2::TOP_LEFT,                   Vector2::TOP_RIGHT,

        Vector2::BOTTOM_LEFT,                Vector2::BOTTOM_RIGHT,
    ];
    pub fn adjacent(self) -> impl Iterator<Item = Vector2> {
        Self::ADJACENT.iter()
            .map(move |&offset| self + offset)
    }
    pub fn around(self) -> impl Iterator<Item = Vector2> {
        Self::AROUND.iter()
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
    
    pub const fn wrap(&self, bounds: Size) -> Vector2 {
        let mut out = *self;
        out.x = out.x.rem_euclid(bounds.width as isize);
        out.y = out.y.rem_euclid(bounds.height as isize);
        out
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
            Direction::North => Vector2::UP,
            Direction::South => Vector2::DOWN,
            Direction::East  => Vector2::RIGHT,
            Direction::West  => Vector2::LEFT,
        }
    }
}

impl From<Direction8> for Vector2 {
    fn from(dir: Direction8) -> Self {
        match dir {
            Direction8::North => Vector2::UP,
            Direction8::South => Vector2::DOWN,
            Direction8::East  => Vector2::RIGHT,
            Direction8::West  => Vector2::LEFT,
            Direction8::NorthWest => Vector2::TOP_LEFT,
            Direction8::NorthEast => Vector2::TOP_RIGHT,
            Direction8::SouthWest => Vector2::BOTTOM_LEFT,
            Direction8::SouthEast => Vector2::BOTTOM_RIGHT,
        }
    }
}

impl From<(isize, isize)> for Vector2 {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
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

impl const Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
// ????? Sub/Neg aren't const traits
// did they just... forget?
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}
impl AddAssign for Vector2 { fn add_assign(&mut self, rhs: Self) { self.x += rhs.x; self.y += rhs.y; } }
impl SubAssign for Vector2 { fn sub_assign(&mut self, rhs: Self) { self.x -= rhs.x; self.y -= rhs.y; } }

impl const Add for &Vector2 { type Output = Vector2; fn add(self, rhs: Self) -> Self::Output { *self + *rhs } }
impl Sub for &Vector2 { type Output = Vector2; fn sub(self, rhs: Self) -> Self::Output { *self - *rhs } }
impl Neg for &Vector2 { type Output = Vector2; fn neg(self) -> Self::Output { -*self } }

impl const Add<Size> for Vector2 {
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
impl AddAssign<Direction> for Vector2 { fn add_assign(&mut self, rhs: Direction) { *self += Self::from(rhs) } }
impl SubAssign<Direction> for Vector2 { fn sub_assign(&mut self, rhs: Direction) { *self -= Self::from(rhs) } }
impl AddAssign<Direction8> for Vector2 { fn add_assign(&mut self, rhs: Direction8) { *self += Self::from(rhs) } }
impl SubAssign<Direction8> for Vector2 { fn sub_assign(&mut self, rhs: Direction8) { *self -= Self::from(rhs) } }

impl Mul<isize> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: isize) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
impl MulAssign<isize> for Vector2 {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl Mul<usize> for Vector2 { type Output = Vector2; fn mul(self, rhs: usize) -> Self::Output { self.mul(rhs as isize) } }
impl MulAssign<usize> for Vector2 { fn mul_assign(&mut self, rhs: usize) { self.mul_assign(rhs as isize) } }
impl Mul<Vector2> for isize { type Output = Vector2; fn mul(self, rhs: Vector2) -> Self::Output { rhs*self } }
impl Mul<Vector2> for usize { type Output = Vector2; fn mul(self, rhs: Vector2) -> Self::Output { rhs*self } }

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
