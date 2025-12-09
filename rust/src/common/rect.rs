use crate::{Vector2, RectIter, Size};
use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Rect {
    base: Vector2,
    size: Size,
}

impl Rect {
    /// Returns [`None`] if the given `size` has zero width or height.
    pub fn new(base: Vector2, size: Size) -> Option<Self> {
        if size.width > 0 && size.height > 0 {
            Some(Self { base, size })
        } else {None}
    }
    pub fn from_origin(size: Size) -> Option<Self> {
        Self::new(Vector2::default(), size)
    }
    pub fn from_corners(a: Vector2, b: Vector2) -> Option<Self> {
        match (a.y <= b.y, a.x <= b.x) {
            // a top left
            (true, true) => Self::from_tl_br(a,b),
            // b top left
            (false, false) => Self::from_tl_br(b,a),
            // tr/bl
            // a top right
            (true, false) => Self::from_tl_br((b.x, a.y).into(), (a.x, b.y).into()),
            // a bottom left
            (false, true) => Self::from_tl_br((a.x, b.y).into(), (b.x, a.y).into()),
        }
    }
    
    pub fn from_tl_br(top_left: Vector2, bottom_right: Vector2) -> Option<Self> {
        if bottom_right.x < top_left.x || bottom_right.y < top_left.y {
            return None;
        }
        let base = top_left;
        let size = (bottom_right - base + Vector2::ONE).try_into().ok()?;
        Some(Self { base, size })
    }
    
    pub const fn width(&self) -> usize { self.size.width }
    pub const fn height(&self) -> usize { self.size.height }
    pub const fn size(&self) -> Size { self.size }
    pub const fn area(&self) -> usize { self.size.width * self.size.height }
    
    pub const fn top_left(&self) -> Vector2 {
        self.base
    }
    pub const fn bottom_right(&self) -> Vector2 {
        self.base + self.size + Vector2::TOP_LEFT
    }
    
    pub const fn x_range(&self) -> Range<isize> {
        self.base.x .. (self.base.x + self.size.width as isize)
    }
    pub const fn y_range(&self) -> Range<isize> {
        self.base.y .. (self.base.y + self.size.height as isize)
    }
    
    pub fn contains(&self, point: &Vector2) -> bool {
        self.x_range().contains(&point.x) && self.y_range().contains(&point.y)
    }
    pub fn inner(&self) -> Option<Rect> {
        Self::from_corners(
            self.top_left() + Vector2::BOTTOM_RIGHT,
            self.bottom_right() + Vector2::TOP_LEFT,
        )
    }
}

impl IntoIterator for Rect {
    type Item = Vector2;
    type IntoIter = RectIter;
    
    fn into_iter(self) -> Self::IntoIter {
        RectIter::new(self)
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

#[test]
fn test_rect_construct() {
    let a: Vector2 = (0,0).into();
    let b: Vector2 = (5,5).into();
    let c: Vector2 = (5,0).into();
    let d: Vector2 = (0,5).into();

    let ab = Rect::from_corners(a, b);
    let ba = Rect::from_corners(b, a);
    let cd = Rect::from_corners(c, d);
    let dc = Rect::from_corners(d,c);
    
    assert!(ab.is_some());
    assert_eq!(ab, ba);
    assert_eq!(ab, cd);
    assert_eq!(ab, dc);
}
