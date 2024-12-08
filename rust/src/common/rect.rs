use crate::{Vector2, RectIter, Size};
use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Rect {
    base: Vector2,
    size: Size,
}

impl Rect {
    pub fn new(base: Vector2, size: Size) -> Option<Self> {
        if size.width > 0 && size.height > 0 {
            Some(Self { base, size })
        } else {None}
    }
    pub fn from_origin(size: Size) -> Option<Self> {
        Self::new(Vector2::default(), size)
    }
    pub fn from_corners(top_left: Vector2, bottom_right: Vector2) -> Option<Self> {
        if bottom_right.x < top_left.x || bottom_right.y < top_left.y {
            return None;
        }
        let base = top_left;
        let size = (bottom_right - base + Vector2 {x: 1, y: 1}).try_into().ok()?;
        Some(Self { base, size })
    }
    
    pub fn width(&self) -> usize { self.size.width }
    pub fn height(&self) -> usize { self.size.height }
    pub fn size(&self) -> Size { self.size }
    
    pub fn top_left(&self) -> Vector2 {
        self.base
    }
    pub fn bottom_right(&self) -> Vector2 {
        self.base - Vector2 {x: 1, y: 1} + self.size
    }
    
    pub fn iter(self) -> RectIter {
        RectIter::new(self)
    }
    
    pub fn x_range(&self) -> Range<isize> {
        self.base.x .. (self.base.x + self.size.width as isize)
    }
    pub fn y_range(&self) -> Range<isize> {
        self.base.y .. (self.base.y + self.size.height as isize)
    }
    
    pub fn contains(&self, point: &Vector2) -> bool {
        self.x_range().contains(&point.x) && self.y_range().contains(&point.y)
    }
}
#[test]
fn test_rect_contains() {
    let rect = Rect {
        base: Vector2 { x: 0, y: 0 },
        size: Size { width: 5, height: 5 }
    };
    let tests: Vec<(isize, isize, bool)> = vec![
        (0,0,true), (0,1,true), (1,0,true), (4,4,true),
        (5,5,false), (-1,-1,false), (-1,0,false), (0,-1,false),
    ];
    for (x,y,should_contain) in tests {
        assert_eq!(should_contain, rect.contains(&Vector2::from((x,y))))
    }
}
