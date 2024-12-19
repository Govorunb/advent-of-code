use std::fmt::{Formatter, Write};
use std::str::FromStr;
use crate::*;

aoc_day!(
    day = 6,
    output = usize,
    examples = [
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
"...##.....
........#.
...^......
.......#..",
"...#
#...
.^..
.#.."
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 41),
            (Self::INPUT, 4903),
        ],
        test_cases![
            (Self::EXAMPLES[0], 6),
            (Self::EXAMPLES[1], 1),
            (Self::EXAMPLES[2], 0),
            (Self::INPUT, 1911),
        ]
    ],
    solve = |input, part| {
        let mut grid: Grid<Symbol> = Grid::from_str(input).unwrap();
        let guard_cell = grid.cells().find(|(_, s)| matches!(s, Symbol::Guard(_))).unwrap();
        let _starting_pos = guard_cell.0;
        let Symbol::Guard(dir) = *guard_cell.1 else {unreachable!()};
        let mut guard = Guard {
            pos: guard_cell.0,
            dir,
            visited: FxHashSet::default(),
        };
        match part {
            Part::One => {
                while Self::step(&grid, &mut guard).is_some() { }
                
                // let mut visit_grid = grid.clone();
                // for visited in &guard.visited {
                //     visit_grid[visited] = Symbol::Visited;
                // }
                // println!("{visit_grid}");
                guard.visited.len()
            },
            Part::Two => {
                let mut route_walker = guard.clone();

                while Self::step(&grid, &mut route_walker).is_some() { }
                
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
);


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

impl Day6 {
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