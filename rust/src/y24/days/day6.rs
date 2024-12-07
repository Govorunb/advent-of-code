use std::fmt::{Formatter, Write};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use crate::*;

pub const DAY6_EXAMPLE: &str =
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub const DAY6_EXAMPLE2: &str =
"\
...##.....
........#.
...^......
.......#..\
";

pub struct Day6 {
    
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Symbol {
    Empty,
    Prop,
    Guard(Direction),
    InsertProp,
    Visited,
    CannotInsertProp,
}

#[derive(Clone, Debug)]
struct Guard {
    pos: Vector2,
    dir: Direction,
    visited: FxHashSet<Vector2>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct TurnAtProp {
    pos: Vector2, // position of the prop
    dir: Direction, // direction facing the prop (before turn)
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            '.' => Symbol::Empty,
            '#' => Symbol::Prop,
            '^' => Symbol::Guard(Direction::North),
            '<' => Symbol::Guard(Direction::West),
            'v' => Symbol::Guard(Direction::South),
            '>' => Symbol::Guard(Direction::East),
            _ => unreachable!(),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Symbol::Empty => '.',
            Symbol::Prop => '#',
            Symbol::Guard(dir) => match dir {
                Direction::North => '^',
                Direction::West => '<',
                Direction::South => 'v',
                Direction::East => '>',
            }
            Symbol::InsertProp => 'O',
            Symbol::CannotInsertProp => '_',
            Symbol::Visited => 'x',
        })?;
        Ok(())
    }
}

impl Day<6> for Day6 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day6.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let mut grid: Grid<Symbol> = Grid::from_str(input).unwrap();
        let guard_cell = grid.cells().find(|(_, s)| matches!(s, Symbol::Guard(_))).unwrap();
        let _starting_pos = guard_cell.0.clone();
        let Symbol::Guard(dir) = *guard_cell.1 else {unreachable!()};
        let mut guard = Guard {
            pos: guard_cell.0,
            dir,
            visited: FxHashSet::default(),
        };
        match part {
            Part::One => {
                while let Some(_) = Self::step(&mut grid, &mut guard) { }
                
                // let mut visit_grid = grid.clone();
                // for visited in &guard.visited {
                //     visit_grid[visited] = Symbol::Visited;
                // }
                // println!("{visit_grid}");
                guard.visited.len()
            },
            Part::Two => {
                let mut route_walker = guard.clone();

                while let Some(_) = Self::step(&mut grid, &mut route_walker) { }
                
                for c in route_walker.visited {
                    // if c == _starting_pos {continue}
                    if grid[&c] != Symbol::Empty {continue}

                    let mut clone = guard.clone();
                    let mut looped = false;
                    let mut turns: FxHashSet<TurnAtProp> = FxHashSet::default();
                    // let mut steps = 0;
                    grid[&c] = Symbol::Prop;
                    while let Some(turn) = Self::step(&grid, &mut clone) {
                        // already seen this turn -> looping
                        if !turns.insert(turn) {
                            looped = true;
                            break;
                        }
                        // came back to start
                        if clone.pos == guard.pos && clone.dir == guard.dir {
                            looped = true;
                            break;
                        }
                        // failsafe
                        // steps += 1;
                        // if steps > 100000 {
                        //     println!("failsafe");
                        //     looped = true;
                        //     break;
                        // }
                    }
                    if looped {
                        grid[&c] = Symbol::InsertProp;
                    } else {
                        grid[&c] = Symbol::CannotInsertProp;
                    }
                }
                
                // println!("{grid}");
                
                grid.elements()
                    .filter(|s| matches!(s, Symbol::InsertProp))
                    .count()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY6_EXAMPLE, 41),
                (self.input(), 4903),
            ],
            test_cases![
                (DAY6_EXAMPLE, 6),
                (DAY6_EXAMPLE2, 1),
                ("...#\n#...\n.^..\n.#..", 0),
                (self.input(), 1911),
            ]
        ]
    }
}

impl Default for Day6 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day6 {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn step(grid: &Grid<Symbol>, guard: &mut Guard) -> Option<TurnAtProp> {
        // move forward until out of bounds/hit a prop
        // possible (micro?)optimization: precompute a coordinate map of all props (like the sparse galaxies grid from y23d11)
        // then just check the row/column once and move there
        grid.ray(guard.pos, guard.dir.into())
            .find_map(|(p, s)| {
                match s {
                    Symbol::Prop => {
                        let turn = TurnAtProp { pos: p, dir: guard.dir };
                        guard.pos = p - guard.dir;
                        guard.dir = guard.dir.turn(Turn::Right);
                        Some(turn)
                    },
                    _ => {
                        guard.visited.insert(p);
                        None
                    }
                }
            })
    }
}