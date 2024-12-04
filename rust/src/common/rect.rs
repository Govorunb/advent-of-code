use crate::{Point, RectIter, Size};
use std::ops::Range;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Rect {
    base: Point,
    size: Size,
}

impl Rect {
    pub fn new(base: Point, size: Size) -> Option<Self> {
        if size.width > 0 && size.height > 0 {
            Some(Self { base, size })
        } else {None}
    }
    pub fn from_origin(size: Size) -> Option<Self> {
        Self::new(Point::default(), size)
    }
    pub fn from_corners(top_left: Point, bottom_right: Point) -> Option<Self> {
        if bottom_right.x < top_left.x || bottom_right.y < top_left.y {
            return None;
        }
        let base = top_left;
        let size = (bottom_right - base.clone() + Point {x: 1, y: 1}).try_into().ok()?;
        Some(Self { base, size })
    }
    
    pub fn width(&self) -> usize { self.size.width }
    pub fn height(&self) -> usize { self.size.height }
    pub fn size(&self) -> Size { self.size }
    
    pub fn top_left(&self) -> Point {
        self.base.clone()
    }
    pub fn bottom_right(&self) -> Point {
        self.base.clone() - Point {x: 1, y: 1} + self.size
    }
    
    pub fn iter(&self) -> RectIter {
        RectIter::new(self.clone())
    }
    
    pub fn x_range(&self) -> Range<isize> {
        self.base.x .. (self.base.x + self.size.width as isize)
    }
    pub fn y_range(&self) -> Range<isize> {
        self.base.y .. (self.base.y + self.size.height as isize)
    }
    
    pub fn contains(&self, point: &Point) -> bool {
        (self.base.x .. self.bottom_right().x).contains(&point.x)
        && (self.base.y .. self.bottom_right().y).contains(&point.y)
    }
}