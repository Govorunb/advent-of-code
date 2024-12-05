use std::slice::Iter;
use std::sync::LazyLock;
use crate::Vector2;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Direction8 {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Turn {
    None,
    Left, // counter-clockwise
    Right, // clockwise
    Opposite,
}

static AROUND: LazyLock<Vec<Vector2>> = LazyLock::new(||
    vec![
        Vector2 {x:-1, y:-1}, Vector2 {x:0,y:-1}, Vector2 {x:1,y:-1},
        Vector2 {x:-1, y: 0},                     Vector2 {x:1,y: 0},
        Vector2 {x:-1, y: 1}, Vector2 {x:0,y: 1}, Vector2 {x:1,y: 1},
    ]
);

static ADJACENT: LazyLock<Vec<Vector2>> = LazyLock::new(||
    vec![
                              Vector2 {x:0,y:-1},
        Vector2 {x:-1, y: 0},                     Vector2 {x:1,y: 0},
                              Vector2 {x:0,y: 1},
    ]
);

static CORNERS: LazyLock<Vec<Vector2>> = LazyLock::new(||
    vec![
        Vector2 {x:-1, y:-1},                     Vector2 {x:1,y:-1},

        Vector2 {x:-1, y: 1},                     Vector2 {x:1,y: 1},
    ]
);

impl Direction {
    pub fn from_delta(dx: i64, dy: i64) -> Self {
        match (dx, dy) {
            (0, -1) => Self::North,
            (0, 1) => Self::South,
            (1, 0) => Self::East,
            (-1, 0) => Self::West,
            _ => unreachable!()
        }
    }

    pub fn move_(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::North => if y > 0 { Some((x, y - 1)) } else { None },
            Self::South => Some((x, y + 1)),
            Self::East  => Some((x + 1, y)),
            Self::West  => if x > 0 { Some((x - 1, y)) } else { None },
        }
    }

    pub fn opp(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    
    pub fn cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    pub fn ccw(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }
    pub fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::None => *self,
            Turn::Left => self.ccw(),
            Turn::Right => self.cw(),
            Turn::Opposite => self.opp(),
        }
    }
    
    pub fn iter() -> Iter<'static, Vector2> {
        ADJACENT.iter()
    }
}

impl Direction8 {
    pub fn from_delta(pt: Vector2) -> Self {
        match (pt.x, pt.y) {
            (-1, -1) => Self::NorthWest,
            (0, -1) => Self::North,
            (1, -1) => Self::NorthEast,
            
            (-1, 0) => Self::West,
            // center
            (1, 0) => Self::East,
            
            (-1, 1) => Self::SouthWest,
            (0, 1) => Self::South,
            (1, 1) => Self::SouthEast,
            _ => unreachable!(),
        }
    }

    pub fn iter() -> Iter<'static, Vector2> {
        AROUND.iter()
    }
    pub fn cardinals() -> Iter<'static, Vector2> {
        Direction::iter()
    }
    pub fn corners() -> Iter<'static, Vector2> {
        CORNERS.iter()
    }
}

impl Turn {
    pub fn from_corner(from: Direction, to: Direction) -> Self {
        match (from, to) {
            (Direction::North, Direction::North) => Self::None,
            (Direction::South, Direction::South) => Self::None,
            (Direction::East, Direction::East) => Self::None,
            (Direction::West, Direction::West) => Self::None,

            (Direction::North, Direction::South) => Self::Opposite,
            (Direction::South, Direction::North) => Self::Opposite,
            (Direction::East, Direction::West) => Self::Opposite,
            (Direction::West, Direction::East) => Self::Opposite,
            
            (Direction::South, Direction::East) => Self::Right,
            (Direction::East, Direction::South) => Self::Left,
            
            (Direction::South, Direction::West) => Self::Left,
            (Direction::West, Direction::South) => Self::Right,
            
            (Direction::North, Direction::East) => Self::Left,
            (Direction::East, Direction::North) => Self::Right,
            
            (Direction::North, Direction::West) => Self::Right,
            (Direction::West, Direction::North) => Self::Left,
        }
    }
}