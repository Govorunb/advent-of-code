use crate::{Direction, Line, RectIter, Size, Vector2};
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
        let (top_left, bottom_right) = match (a.y <= b.y, a.x <= b.x) {
            // a top left
            (true, true) => (a,b),
            // b top left
            (false, false) => (b,a),
            // tr/bl
            // a top right
            (true, false) => ((b.x, a.y).into(), (a.x, b.y).into()),
            // a bottom left
            (false, true) => ((a.x, b.y).into(), (b.x, a.y).into()),
        };
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
    pub const fn area(&self) -> usize { self.width() * self.height() }
    pub const fn perimeter(&self) -> usize { self.width() * 2 + self.height() * 2}
    
    pub const fn top_left(&self) -> Vector2 {
        self.base
    }
    pub const fn bottom_right(&self) -> Vector2 {
        self.base + self.size + Vector2::TOP_LEFT
    }

    pub const fn top_right(&self) -> Vector2 {
        self.base + Vector2::RIGHT * (self.width()-1)
    }

    pub const fn bottom_left(&self) -> Vector2 {
        self.base + Vector2::DOWN * (self.height()-1)
    }
    pub const fn corners_cw(&self) -> [Vector2; 4] {
        [self.top_left(),self.top_right(),self.bottom_right(),self.bottom_left()]
    }
    
    pub const fn x_range(&self) -> Range<isize> {
        self.base.x .. (self.base.x + self.size.width as isize)
    }
    pub const fn y_range(&self) -> Range<isize> {
        self.base.y .. (self.base.y + self.size.height as isize)
    }
    
    pub fn contains(&self, point: &Vector2) -> bool {
        point.x >= self.top_left().x && point.x <= self.bottom_right().x
        && point.y >= self.top_left().y && point.y <= self.bottom_right().y
    }

    pub fn inset(&self, amount: isize) -> Option<Rect> {
        Self::from_corners(
            self.top_left() + Vector2::BOTTOM_RIGHT * amount,
            self.bottom_right() + Vector2::TOP_LEFT * amount,
        )
    }

    pub fn top_edge(&self) -> Line {
        Line {
            origin: self.base,
            dir: Direction::East,
            len: self.width()
        }
    }

    pub fn right_edge(&self) -> Line {
        Line {
            origin: self.top_right(),
            dir: Direction::South,
            len: self.height()
        }
    }

    pub fn bottom_edge(&self) -> Line {
        Line {
            origin: self.bottom_right(),
            dir: Direction::West,
            len: self.width()
        }
    }

    pub fn left_edge(&self) -> Line {
        Line {
            origin: self.bottom_left(),
            dir: Direction::North,
            len: self.height()
        }
    }

    pub fn sides(&self) -> [Line;4] {
        [self.top_edge(), self.right_edge(), self.bottom_edge(), self.left_edge()]
    }

    pub fn intersects(&self, line: &Line) -> bool {
        self.top_edge().intersects(line)
        || self.bottom_edge().intersects(line)
        || self.left_edge().intersects(line)
        || self.right_edge().intersects(line)
    }

    pub fn left(&self) -> isize {
        self.base.x
    }
    pub fn top(&self) -> isize {
        self.base.y
    }
    pub fn right(&self) -> isize {
        self.bottom_right().x
    }
    pub fn bottom(&self) -> isize {
        self.bottom_right().y
    }

    pub fn overlaps(&self, rect: &Rect) -> bool {
        // classic AABB collision test
        self.left() < rect.left()
        && self.right() > rect.left()
        && self.top() < rect.bottom()
        && self.bottom() > rect.top()
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

#[test]
fn test_rect_contains() {
    let rect = Rect::from_origin((4,4).into()).unwrap();
    let tests: Vec<(isize, isize, bool)> = vec![
        (0,0,true), (0,1,true), (1,0,true), (4,4,true),
        (5,5,false), (-1,-1,false), (-1,0,false), (0,-1,false),
    ];
    for (x,y,should_contain) in tests {
        assert_eq!(should_contain, rect.contains(&(x,y).into()), "{x},{y},{should_contain}");
    }
}

#[test]
fn rect_intersects() {
    let rect = Rect::from_origin((10,10).into()).unwrap();
    let tests: Vec<([isize; 4], bool)> = vec![
        ([-1,-1, -1,900], false),
        ([0,0, 0,900], true),
    ];
    for case in tests {
        let ([ax,ay,bx,by], expected) = case;
        let a = (ax,ay).into();
        let b = (bx,by).into();
        let line = Line::new(a, b).unwrap();
        assert_eq!(expected, rect.intersects(&line), "{case:?}");
    }
}