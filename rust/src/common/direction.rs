use std::fmt::{Display, Formatter};
use std::ops::Neg;
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

impl Direction {
    pub const fn opp(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
    
    pub const fn cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    pub const fn ccw(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }
    pub const fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::None => *self,
            Turn::Left => self.ccw(),
            Turn::Right => self.cw(),
            Turn::Opposite => self.opp(),
        }
    }

    pub const fn sides(&self) -> [Direction; 2] {
        [self.turn(Turn::Left), self.turn(Turn::Right)]
    }
    
    pub const fn all_clockwise() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }
    
    pub const fn all_counterclockwise() -> [Direction;4] {
        [Direction::North, Direction::West, Direction::South, Direction::East]
    }

    pub const fn parse(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Self::North),
            'v' => Some(Self::South),
            '>' => Some(Self::East),
            '<' => Some(Self::West),
            _ => None,
        }
    }

    pub const fn to_vec2(self) -> Vector2 {
        Vector2::from(self)
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        Self::parse(c).expect("Unmatched char {c} in Direction::from(char)")
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::North => '^',
            Self::East => '>',
            Self::South => 'v',
            Self::West => '<'
        })
    }
}

impl Direction8 {
    pub const fn opp(&self) -> Direction8 {
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
    
    pub const fn all_clockwise() -> [Direction8;8] {
        [
            Self::North, Self::NorthEast,
            Self::East, Self::SouthEast,
            Self::South, Self::SouthWest,
            Self::West, Self::NorthWest
        ]
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
    pub const fn from_corner(from: Direction, to: Direction) -> Self {
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

    pub const fn parse(c: char) -> Option<Turn> {
        match c {
            'r' | 'R' | '>' => Some(Self::Right),
            'l' | 'L' | '<' => Some(Self::Left),
            // 'a' | 'A' | 'o' | 'O' => Some(Self::Opposite),
            _ => None,
        }
    }
}

impl From<char> for Turn {
    fn from(c: char) -> Self {
        Self::parse(c).expect("Unmatched char {c} in Direction::from(char)")
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
