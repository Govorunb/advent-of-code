use std::str::FromStr;
use crate::*;

pub struct Day10 {
    
}

struct PipeGrid {
    tiles: Grid<Symbol>,
    pipes: Vec<Tile>, // "linked list"
    animal: Vector2,
    pipe_under_animal: Symbol,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Tile {
    coords: Vector2,
    symbol: Symbol,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Symbol {
    Empty, // .
    Pipe(Pipe),
    Animal, // S
}
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Pipe {
    Vertical, // |
    Horizontal, // -
    TL, // F - south and east
    TR, // 7 - south and west
    BL, // L - north and east
    BR, // J - north and west
}

impl Pipe {
    fn connections(&self) -> (Direction, Direction) {
        match self {
            Self::Vertical => (Direction::North, Direction::South),
            Self::Horizontal => (Direction::West, Direction::East),
            Self::TL => (Direction::South, Direction::East),
            Self::TR => (Direction::South, Direction::West),
            Self::BL => (Direction::North, Direction::East),
            Self::BR => (Direction::North, Direction::West),
        }
    }
    fn traverse(&self, came_from: Direction) -> Direction {
        let (one, other) = self.connections();
        if one == came_from {
            other
        } else {
            one
        }
    }
    fn is_connected(&self, dir: Direction) -> bool {
        let (one, other) = self.connections();
        one == dir || other == dir
    }

    fn from_conn(dir1: Direction, dir2: Direction) -> Self {
        match (dir1, dir2) {
            (Direction::North, Direction::South) => Self::Vertical,
            (Direction::South, Direction::North) => Self::Vertical,
            
            (Direction::East, Direction::West) => Self::Horizontal,
            (Direction::West, Direction::East) => Self::Horizontal,
            
            (Direction::South, Direction::East) => Self::TL,
            (Direction::East, Direction::South) => Self::TL,
            
            (Direction::South, Direction::West) => Self::TR,
            (Direction::West, Direction::South) => Self::TR,
            
            (Direction::North, Direction::East) => Self::BL,
            (Direction::East, Direction::North) => Self::BL,
            
            (Direction::North, Direction::West) => Self::BR,
            (Direction::West, Direction::North) => Self::BR,
            
            _ => unreachable!(),
        }
    }
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            '.' => Symbol::Empty,
            '|' => Symbol::Pipe(Pipe::Vertical),
            '-' => Symbol::Pipe(Pipe::Horizontal),
            'F' => Symbol::Pipe(Pipe::TL),
            '7' => Symbol::Pipe(Pipe::TR),
            'L' => Symbol::Pipe(Pipe::BL),
            'J' => Symbol::Pipe(Pipe::BR),
            'S' => Symbol::Animal,
            _ => unreachable!(),
        }
    }
}

impl PipeGrid {
    fn parse(input: &str) -> PipeGrid {
        let tiles = Grid::from_str(input).unwrap();
        let animal = tiles.cells()
            .find(|&(_, &c)| matches!(c, Symbol::Animal)).unwrap()
            .0;

        let mut grid = PipeGrid {
            tiles,
            pipes: vec![],
            animal,
            pipe_under_animal: Symbol::Animal, // placeholder
        };
        grid.discover_pipe_network();
        grid
    }
    fn discover_pipe_network(&mut self) {
        let mut graph: Vec<Tile> = vec![];
        
        // infer what pipe under animal is from what it's connected to
        let mut first_pipe = None;
        for dir in Direction::all_clockwise() {
            let moved = self.animal + dir;
            if let Some(Symbol::Pipe(p)) = self.tiles.get(&moved) {
                if p.is_connected(dir.opp()) {
                    match first_pipe {
                        None => first_pipe = Some((moved, dir.opp())),
                        Some((_, d)) => {
                            self.pipe_under_animal = Symbol::Pipe(Pipe::from_conn(d, dir.opp()));
                            break;
                        }
                    };
                }
            } else {continue}
        }
        let (mut pt, mut dir) = first_pipe.expect("did not find first pipe");
        let mut curr_tile: Option<&Symbol> = self.tiles.get(&pt);
        while let Some(Symbol::Pipe(p)) = curr_tile {
            graph.push(Tile {
                coords: pt,
                symbol: Symbol::Pipe(*p),
            });
            dir = p.traverse(dir);
            pt += dir;
            dir = dir.opp();
            curr_tile = self.tiles.get(&pt);
        }
        
        self.pipes = graph;
    }

    fn inside_pipe_network(&self) -> usize {
        let mut partition_r: FxHashSet<Vector2> = FxHashSet::default();
        let mut partition_l: FxHashSet<Vector2> = FxHashSet::default();
        
        let mut turns = 0;
        let delta = self.animal - self.pipes[0].coords;
        let mut dir = delta.try_into().unwrap();
        for pipe_tile in self.pipes.as_slice() {
            let pipe = match pipe_tile.symbol {
                Symbol::Pipe(pipe) => pipe,
                _ => unreachable!()
            };
            let new_dir = pipe.traverse(dir);
            let turn = Turn::from_corner(dir, new_dir);
            match turn {
                Turn::Left => turns -= 1,
                Turn::Right => turns += 1,
                _ => (),
            };
            
            let pt = pipe_tile.coords;
            
            if turn == Turn::None {
                let cw = pt + new_dir.cw();
                let ccw = pt + new_dir.ccw();
                for (p, par) in [(cw, &mut partition_r), (ccw, &mut partition_l)] {
                    if self.tiles.flat_index(&p).is_some()
                        && !self.pipes.iter().any(|&p_| p_.coords == p) {
                        par.extend(self.flood_fill(&cw, par));
                    }
                }
            } else {
                let dir_past = dir.opp();
                let dir_behind = new_dir.opp();
                
                let outside_is_l = turn == Turn::Right;
                let partition = if outside_is_l { &mut partition_l } else { &mut partition_r };

                let past = pt + dir_past;
                let behind = pt + dir_behind;
                for p in [past, behind] {
                    if self.tiles.bounds().contains(&p)
                        && !self.pipes.iter().any(|&p_| p_.coords == p)
                        && self.animal != p {
                        partition.extend(self.flood_fill(&p, partition));
                    }
                }
            }

            dir = new_dir.opp();
        }
        let cw_is_inside = turns > 0;
        //println!("{cw_is_inside} {} {}", partition_r.len(), partition_l.len());

        let visited = if cw_is_inside {
            partition_r
        } else {
            partition_l
        };

        visited.len()
    }

    fn flood_fill(&self, pt: &Vector2, visited: &FxHashSet<Vector2>) -> Vec<Vector2> {
        if visited.contains(pt) {
            return vec![];
        }
        
        flood_fill_adjacent(pt, |_, &adj| {
            self.tiles.bounds().contains(&adj)
                && !self.pipes.iter().any(|t| t.coords == adj)
        })
    }
}

impl Day<10> for Day10 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day10.txt");

    fn solve_part(&self, input: &str, part: Part) -> usize {
        let grid = PipeGrid::parse(input);
        match part {
            Part::One => {
                let pipes = grid.pipes.len();
                ((1. + pipes as f64) / 2.).ceil() as usize
            },
            Part::Two => {
                grid.inside_pipe_network()
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 4),
                (Self::EXAMPLES[1], 8),
                (Self::INPUT, 6768),
            ],
            test_cases![
                (Self::EXAMPLES[2], 4),
                (Self::EXAMPLES[3], 4),
                (Self::EXAMPLES[4], 8),
                (Self::EXAMPLES[5], 10),
            ]
        ]
    }
}

impl Default for Day10 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day10 {
    pub fn new() -> Self {
        Self {
        }
    }
}