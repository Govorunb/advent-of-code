#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub enum Direction8 {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Turn {
    None,
    Left,
    Right,
    Opposite,
}

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