use crate::*;


/// Axis-aligned line (i.e. horizontal or vertical).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    pub origin: Vector2,
    pub dir: Direction,
    pub len: usize,
}

impl Line {
    pub fn new(p1: Vector2, p2: Vector2) -> Option<Self> {
        Direction::try_from(p2-p1)
            .ok().map(|dir| Self {
                dir,
                origin: p1,
                len: 1+p2.manhattan_distance(p1)
            })
    }

    pub fn from_origin(p: Vector2) -> Option<Self> {
        Self::new((0,0).into(), p)
    }

    pub fn contains(&self, pt: &Vector2) -> bool {
        *pt == self.origin || Line::new(self.origin, *pt)
            .is_some_and(|other| other.dir == self.dir && other.len <= self.len)
    }

    pub fn iter(&self) -> impl Iterator<Item = Vector2> {
        self.origin.ray(self.dir.to_vec2()).take(self.len)
    }

    pub fn into_iter(self) -> impl Iterator<Item = Vector2> {
        self.origin.ray(self.dir.to_vec2()).take(self.len)
    }

    pub fn shift(self, amt: isize) -> Self {
        Self {
            dir: self.dir,
            len: self.len,
            origin: self.origin + self.dir.to_vec2() * amt
        }
    }

    pub fn start(&self) -> Vector2 {
        self.origin
    }

    pub fn end(&self) -> Vector2 {
        self.origin + self.dir.to_vec2() * (self.len-1)
    }

    pub fn points(self) -> (Vector2, Vector2) {
        (self.start(), self.end())
    }

    pub fn intersects(&self, other: &Line) -> bool {
        match (self.dir.is_vertical(), other.dir.is_vertical()) {
            (true, true) | (false, false) => self.contains(&other.origin) || other.contains(&self.origin),
            (true, false) => {
                let pt_intersect = (self.origin.x, other.origin.y).into();
                self.contains(&pt_intersect) && other.contains(&pt_intersect)
            },
            (false, true) => {
                let pt_intersect = (other.origin.x, self.origin.y).into();
                self.contains(&pt_intersect) && other.contains(&pt_intersect)
            },
        }
    }

    pub fn connect_successive_pts(pts: &[Vector2]) -> impl Iterator<Item = Line> {
        pts.iter()
            .cycle().tuple_windows().take(pts.len())
            .map(|(&a, &b)| Line::new(a, b).unwrap())
    }

    pub fn rev(self) -> Self {
        Self::new(self.end(), self.start())
            .expect("Line.rev() cloned into a None - did you consume the line as an iterator or manually set len=0?")
    }
}

impl Iterator for Line {
    type Item = Vector2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {return None};
        self.len -= 1;
        let pt = self.origin;
        self.origin += self.dir.to_vec2();
        Some(pt)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl ExactSizeIterator for Line {
    fn len(&self) -> usize {
        self.len
    }
}

#[test]
fn line_contains() {
    let line = Line::from_origin((0,10).into()).unwrap();
    let tests: Vec<(isize, isize, bool)> = vec![
        (-1, -1, false), (-1, 0, false), (0, -1, false),
        (0, 0, true), (0, 1, true), (0, 10, true),
        (0, 11, false), (1, 0, false), (10, 0, false),
    ];
    for &(x, y, should_contain) in &tests {
        assert_eq!(should_contain, line.contains(&(x,y).into()), "1 {x},{y},{should_contain}");
    }
    let line_rev = Line::new((0,10).into(), (0,0).into()).unwrap();
    for &(x, y, should_contain) in &tests {
        assert_eq!(should_contain, line_rev.contains(&(x,y).into()), "2 {x},{y},{should_contain}");
    }
}

#[test]
fn line_rev() {
    let line = Line::from_origin((0,10).into()).unwrap();
    assert_eq!(line.rev(), Line::new((0,10).into(), (0,0).into()).unwrap());
}

#[test]
fn line_intersects() {
    let line = Line::from_origin((0,10).into()).unwrap();
    let tests: Vec<([isize; 4], bool)> = vec![
        ([-1,-1, -1,0], false), ([-1,-1, 0,-1], false),
        ([0,0, 0,1], true), ([0,0, 0,10], true), ([10,0, 0,0], true),
        ([0,10, 0,11], true), ([0,0, 1,0], true),
        ([5,5, 5,6], false), ([-5,5, 5,5], true), ([-5,0, 10,0], true),
    ];
    for case in tests {
        let ([x1,y1,x2,y2], expected) = case;
        let other = Line::new((x1,y1).into(),(x2,y2).into()).unwrap();
        assert_eq!(expected, line.intersects(&other), "{case:?}");
    }

    let line2 = Line::new((10,10).into(), (0,10).into()).unwrap();
    assert!(!line2.intersects(&Line::new((-1,-1).into(), (-1,900).into()).unwrap()));
}

#[test]
fn line_intersects_rect() {
    let rect = Rect::from_origin((10,10).into()).unwrap();
    let tests: Vec<([isize; 4], &str)> = vec![
        ([-1,-1, -1,900], "none"),
    ];
    for case in tests {
        let ([ax,ay,bx,by], expected) = case;
        let a = (ax,ay).into();
        let b = (bx,by).into();
        let line = Line::new(a, b).unwrap();

        let edges = [
            (rect.top_edge(), "top"),
            (rect.right_edge(), "right"),
            (rect.bottom_edge(), "bottom"),
            (rect.left_edge(), "left"),
        ];

        let intersecting_edge = edges.iter()
            .find(|(edge, _)| edge.intersects(&line))
            .map(|&(_, name)| name)
            .unwrap_or("none");
        assert_eq!(expected, intersecting_edge, "{case:?} {intersecting_edge}");
    }
}
