use std::fmt::Formatter;
use std::str::FromStr;
use crate::*;

pub struct Day15 {
    
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
enum Symbol {
    #[default]
    Empty,
    Wall,
    Box,
    BoxL,
    BoxR,
    Robot
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            '.' => Symbol::Empty,
            '#' => Symbol::Wall,
            'O' => Symbol::Box,
            '@' => Symbol::Robot,
            '[' => Symbol::BoxL,
            ']' => Symbol::BoxR,
            _ => unreachable!()
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Symbol::Empty => '.',
            Symbol::Wall => '#',
            Symbol::Box => 'O',
            Symbol::Robot => '@',
            Symbol::BoxL => '[',
            Symbol::BoxR => ']',
        })
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => unreachable!()
        }
    }
}

impl Day<15> for Day15 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day15.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let (grid_s, moves_s) = input.split_once("\n\n").unwrap();
        let moves: Vec<Direction> = moves_s.chars()
            .filter(|&c| c != '\n')
            .map(Direction::from)
            .collect_vec();
        let mut grid = Grid::from_str(grid_s).unwrap();
        if part == Part::Two {
            let mut p2_grid = Grid::from_origin(
                Size {
                    width: grid.width()*2,
                    height: grid.height()
                }
            ).unwrap();
            for (mut p, c) in grid.cells() {
                p.x *= 2;
                p2_grid[p] = match c {
                    Symbol::Wall => Symbol::Wall,
                    Symbol::Box => Symbol::BoxL,
                    Symbol::Empty => Symbol::Empty,
                    Symbol::Robot => Symbol::Robot,
                    _ => unreachable!()
                };
                p2_grid[p + Vector2::RIGHT] = match c {
                    Symbol::Wall => Symbol::Wall,
                    Symbol::Box => Symbol::BoxR,
                    Symbol::Empty | Symbol::Robot => Symbol::Empty,
                    _ => unreachable!()
                }
            }
            grid = p2_grid;
        }
        let mut robot_pos = grid.cells()
            .find(|&c| *c.1 == Symbol::Robot)
            .unwrap().0;
        for dir in moves {
            // safety clone - if only one side of BoxL/BoxR can move, that side will push and we can't roll that back
            // i wrote a version that didn't clone but it was like 100 more LoC and 10x less readable, i think i'll take the 3ms hit
            let backup = grid.clone();
            if Self::try_move(&mut grid, robot_pos, dir) {
                // println!("before moving ({robot_pos})->{dir:?}:\n{grid}");
                robot_pos += dir;
                // println!("after moving ({robot_pos})->{dir:?}:\n{grid}");
            } else {
                grid = backup; // if you reach this line, you can put "restored backups" on your resume
            }
        }
        
        // println!("{grid}");
        
        grid.cells()
            .filter(|(_,s)| matches!(s, Symbol::Box | Symbol::BoxL))
            .map(|(p,_)| Self::gps(p))
            .sum()
    }
    const EXAMPLES: &'static [&'static str] = &[
"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 2028),
                (Self::EXAMPLES[1], 10092),
                (Self::INPUT, 1559280),
            ],
            test_cases![
                (Self::EXAMPLES[1], 9021),
                (Self::INPUT, 1576353),
            ]
        ]
    }
}

impl Default for Day15 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day15 {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn gps(pos: Vector2) -> usize {
        (pos.x + 100 * pos.y) as usize
    }
    
    fn move_unchecked(grid: &mut Grid<Symbol>, pos: Vector2, dir: Direction) {
        let sym = grid[pos];
        grid[pos] = Symbol::Empty;
        grid[pos + dir] = sym;
    }
    
    fn try_move(grid: &mut Grid<Symbol>, pos: Vector2, dir: Direction) -> bool {
        let sym = grid[pos];
        match sym {
            Symbol::Empty => true,
            Symbol::Wall => false,
            Symbol::Robot | Symbol::Box => {
                let can_move = Self::try_move(grid, pos+dir, dir);
                if can_move {
                    Self::move_unchecked(grid, pos, dir);
                }
                can_move
            },
            Symbol::BoxL => {
                match dir {
                    Direction::North | Direction::South => {
                        let can_move_l = Self::try_move(grid, pos+dir, dir);
                        let can_move_r = Self::try_move(grid, pos+Vector2::RIGHT+dir, dir);
                        if can_move_l && can_move_r {
                            Self::move_unchecked(grid, pos, dir);
                            Self::move_unchecked(grid, pos+Vector2::RIGHT, dir);
                        }
                        can_move_l && can_move_r
                    },
                    Direction::East => Self::try_move(grid, pos+dir, dir),
                    Direction::West => {
                        let can_move = Self::try_move(grid, pos+dir, dir);
                        if can_move {
                            Self::move_unchecked(grid, pos, dir);
                            Self::move_unchecked(grid, pos+Vector2::RIGHT, dir);
                        }
                        can_move
                    },
                }
            },
            Symbol::BoxR => {
                match dir {
                    Direction::North | Direction::South => {
                        let can_move_l = Self::try_move(grid, pos+Vector2::LEFT+dir, dir);
                        let can_move_r = Self::try_move(grid, pos+dir, dir);
                        if can_move_l && can_move_r {
                            Self::move_unchecked(grid, pos+Vector2::LEFT, dir);
                            Self::move_unchecked(grid, pos, dir);
                        }
                        can_move_l && can_move_r
                    },
                    Direction::West => Self::try_move(grid, pos+dir, dir),
                    Direction::East => {
                        let can_move = Self::try_move(grid, pos+dir, dir);
                        if can_move {
                            Self::move_unchecked(grid, pos, dir);
                            Self::move_unchecked(grid, pos+Vector2::LEFT, dir);
                        }
                        can_move
                    },
                }
            }
        }
    }
}