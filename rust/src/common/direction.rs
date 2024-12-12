use std::iter::Rev;
use std::ops::Neg;
use std::slice::Iter;
use std::sync::LazyLock;
use itertools::Either;
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
        Vector2 {x:-1,y:-1}, Vector2 {x: 0,y:-1}, Vector2 {x: 1,y:-1},
        Vector2 {x:-1,y: 0},                      Vector2 {x: 1,y: 0},
        Vector2 {x:-1,y: 1}, Vector2 {x: 0,y: 1}, Vector2 {x: 1,y: 1},
    ]
);

static ADJACENT: LazyLock<Vec<Vector2>> = LazyLock::new(||
    vec![
                             Vector2 {x: 0,y:-1},
        Vector2 {x:-1,y: 0},                      Vector2 {x: 1,y: 0},
                             Vector2 {x: 0,y: 1},
    ]
);

static CORNERS: LazyLock<Vec<Vector2>> = LazyLock::new(||
    vec![
        Vector2 {x:-1,y:-1},                      Vector2 {x: 1,y:-1},

        Vector2 {x:-1,y: 1},                      Vector2 {x: 1,y: 1},
    ]
);

impl Direction {
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

    pub fn sides(&self) -> [Direction; 2] {
        match self {
            Direction::North | Direction::South => [Direction::West, Direction::East],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }
    
    pub fn all_clockwise() -> Iter<'static, Direction> {
        [Direction::North, Direction::East, Direction::South, Direction::West].iter()
    }
    
    pub fn all_counterclockwise() -> Rev<Iter<'static, Direction>> {
        Self::all_clockwise().rev()
    }
    
    pub fn deltas() -> Iter<'static, Vector2> {
        ADJACENT.iter()
    }
}

impl Direction8 {
    pub fn opp(&self) -> Direction8 {
        match self {
            Self::North => Self::South,
            Self::West => Self::East,
            Self::East => Self::West,
            Self::South => Self::North,
            
            Self::NorthWest => Self::SouthEast,
            Self::NorthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthEast,
            Self::SouthEast => Self::NorthWest,
        }
    }
    
    pub fn all_clockwise() -> Iter<'static, Direction8> {
        [
            Self::North, Self::NorthEast,
            Self::East, Self::SouthEast,
            Self::South, Self::SouthWest,
            Self::West, Self::NorthWest
        ].iter()
    }
    pub fn deltas() -> Iter<'static, Vector2> {
        AROUND.iter()
    }
    pub fn cardinal_deltas() -> Iter<'static, Vector2> {
        Direction::deltas()
    }
    pub fn corner_deltas() -> Iter<'static, Vector2> {
        CORNERS.iter()
    }
}

impl Neg for Direction8 {
    type Output = Direction8;
    fn neg(self) -> Self::Output {self.opp()}
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Self::Output {self.opp()}
}

impl Turn {
    pub fn from_corner(from: Direction, to: Direction) -> Self {
        // just hoping this compiles to a LUT
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

#[derive(Debug)]
pub struct CenterNoDirection;
#[derive(Debug)]
pub struct DirectionNotCardinal;
impl TryFrom<Vector2> for Direction {
    type Error = Either<CenterNoDirection, DirectionNotCardinal>;

    fn try_from(pt: Vector2) -> Result<Self, Self::Error> {
        let x = pt.x.signum();
        let y = pt.y.signum();
        match (x, y) {
            ( 0,-1) => Ok(Self::North),
            ( 0, 1) => Ok(Self::South),
            (-1, 0) => Ok(Self::West),
            ( 1, 0) => Ok(Self::East),
            
            ( 0, 0) => Err(Either::Left(CenterNoDirection)),
            _ => Err(Either::Right(DirectionNotCardinal)),
        }
    }
}
impl TryFrom<Vector2> for Direction8 {
    type Error = CenterNoDirection;

    fn try_from(pt: Vector2) -> Result<Self, Self::Error> {
        let x = pt.x.signum();
        let y = pt.y.signum();
        match (x, y) {
            (-1, -1) => Ok(Self::NorthWest),
            ( 0, -1) => Ok(Self::North),
            ( 1, -1) => Ok(Self::NorthEast),

            (-1,  0) => Ok(Self::West),
            ( 0,  0) => Err(CenterNoDirection),
            ( 1,  0) => Ok(Self::East),

            (-1,  1) => Ok(Self::SouthWest),
            ( 0,  1) => Ok(Self::South),
            ( 1,  1) => Ok(Self::SouthEast),
            
            _ => unreachable!(),
        }
    }
}
