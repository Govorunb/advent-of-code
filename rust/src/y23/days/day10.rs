use crate::test_cases;
use crate::common::*;

pub const DAY10_INPUT: &str = include_str!("../Input/day10.txt");
pub const DAY10_EXAMPLE1: &str =
"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
pub const DAY10_EXAMPLE2: &str =
"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
pub const DAY10_EXAMPLE3: &str =
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
pub const DAY10_EXAMPLE3_1: &str =
"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
pub const DAY10_EXAMPLE4: &str =
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
pub const DAY10_EXAMPLE5: &str =
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

pub struct Day10 {
    
}

struct PipeGrid {
    tiles: Vec<Tile>, // this should be a Grid<Tile>
    pipes: Vec<Tile>,
    width: usize,
    height: usize,
    animal: (usize, usize),
    pipe_under_animal: Tile,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Tile {
    x: usize,
    y: usize,
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

impl PipeGrid {
    fn parse(input: &str) -> PipeGrid {
        let lines = input.lines().collect_vec();
        let mut tiles = Vec::new();
        let width = lines[0].len();
        let height = lines.len();
        let mut animal = (0, 0);

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.char_indices() {
                let symbol = match c {
                    '.' => Symbol::Empty,
                    '|' => Symbol::Pipe(Pipe::Vertical),
                    '-' => Symbol::Pipe(Pipe::Horizontal),
                    'F' => Symbol::Pipe(Pipe::TL),
                    '7' => Symbol::Pipe(Pipe::TR),
                    'L' => Symbol::Pipe(Pipe::BL),
                    'J' => Symbol::Pipe(Pipe::BR),
                    'S' => Symbol::Animal,
                    _ => unreachable!(),
                };
                if let Symbol::Animal = symbol {
                    animal = (x, y);
                }
                tiles.push(Tile {
                    x,
                    y,
                    symbol,
                });
            }
        }

        let mut grid = PipeGrid {
            tiles,
            pipes: vec![],
            width,
            height,
            animal,
            pipe_under_animal: Tile {
                x: animal.0,
                y: animal.1,
                symbol: Symbol::Animal,
            }, // placeholder
        };
        grid.discover_pipe_network();
        grid
    }
    fn discover_pipe_network(&mut self) {
        let mut graph: Vec<Tile> = vec![];
        let (start_x, start_y) = self.animal;
        
        let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
        let mut first_pipe = None;
        for dir in directions {
            if let Some((x, y)) = dir.move_(start_x, start_y) {
                match self.get(x, y).symbol {
                    Symbol::Pipe(p) => {
                        if p.is_connected(dir.opp()) {
                            if first_pipe.clone().is_none() {
                                first_pipe = Some((x, y, dir.opp()));
                            } else {
                                self.pipe_under_animal.symbol = Symbol::Pipe(Pipe::from_conn(first_pipe.unwrap().2, dir.opp()));
                                break;
                            }
                        }
                    },
                    _ => {continue;}
                }
            }
        }
        let (mut x, mut y, mut dir) = first_pipe.expect("did not find first pipe");
        let mut curr_tile = self.get(x, y);
        while let Symbol::Pipe(p) = curr_tile.symbol {
            graph.push(Tile {
                x,
                y,
                symbol: Symbol::Pipe(p),
            });
            dir = p.traverse(dir);
            (x, y) = dir.move_(x, y).unwrap();
            dir = dir.opp();
            curr_tile = self.get(x,y);
        }
        
        self.pipes = graph;
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[self.index(x, y)]
    }

    fn inside_pipe_network(&self) -> usize {
        let mut partition_r: FxHashSet<&Tile> = FxHashSet::default();
        let mut partition_l: FxHashSet<&Tile> = FxHashSet::default();
        
        let mut turns = 0;
        let dx = self.pipe_under_animal.x as i64 - self.pipes[0].x as i64;
        let dy = self.pipe_under_animal.y as i64 - self.pipes[0].y as i64;
        let mut dir = Direction::from_delta(dx, dy);
        for p in self.pipes.as_slice() {
            let pipe = match p.symbol {
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
            
            if turn == Turn::None {
                let cw_maybe = new_dir.cw().move_(p.x, p.y);
                let ccw_maybe = new_dir.ccw().move_(p.x, p.y);
                if let Some(cw) = cw_maybe {
                    if (0..=self.tiles.len()-1).contains(&self.index(cw.0, cw.1))
                    && !self.pipes.contains(self.get(cw.0, cw.1)) {
                        partition_r.extend(self.flood_fill(cw.0, cw.1, &partition_r));
                    }
                }
                if let Some(ccw) = ccw_maybe {
                    if (0..=self.tiles.len()-1).contains(&self.index(ccw.0, ccw.1))
                        && !self.pipes.contains(self.get(ccw.0, ccw.1)) {
                        partition_l.extend(self.flood_fill(ccw.0, ccw.1, &partition_l));
                    }
                }
            } else {
                let dir_past = dir.opp();
                let dir_behind = new_dir.opp();
                
                let outside_is_l = turn == Turn::Right;
                let partition = if outside_is_l { &mut partition_l } else { &mut partition_r };

                let past_maybe = dir_past.move_(p.x, p.y);
                let behind_maybe = dir_behind.move_(p.x, p.y);
                if let Some(past) = past_maybe {
                    if (0..=self.tiles.len()-1).contains(&self.index(past.0, past.1))
                        && !self.pipes.contains(self.get(past.0, past.1))
                        && !(self.animal.0 == past.0 && self.animal.1 == past.1) {
                        partition.extend(self.flood_fill(past.0, past.1, partition));
                    }
                }
                if let Some(behind) = behind_maybe {
                    if (0..=self.tiles.len()-1).contains(&self.index(behind.0, behind.1))
                        && !self.pipes.contains(self.get(behind.0, behind.1))
                        && !(self.animal.0 == behind.0 && self.animal.1 == behind.1) {
                        partition.extend(self.flood_fill(behind.0, behind.1, partition));
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
    
    #[allow(dead_code)] // forgor
    fn around(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut around: Vec<(usize, usize)> = vec![];
        around.extend(self.adjacent(x, y));
        let (left, up) = (x > 0, y > 0);
        let (right, down) = (x < self.width - 1, y < self.height - 1);
        if left && up {
            around.push((x - 1, y - 1));
        }
        if right && up {
            around.push((x + 1, y - 1));
        }
        if left && down {
            around.push((x - 1, y + 1));
        }
        if right && down {
            around.push((x + 1, y + 1));
        }
        around
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacent: Vec<(usize, usize)> = vec![];
        let (left, up) = (x > 0, y > 0);
        let (right, down) = (x < self.width - 1, y < self.height - 1);
        if up {
            adjacent.push((x, y - 1));
        }
        if left {
            adjacent.push((x - 1, y));
        }
        if right {
            adjacent.push((x + 1, y));
        }
        if down {
            adjacent.push((x, y + 1));
        }
        adjacent
    }

    fn flood_fill<'a>(&'a self, x: usize, y: usize, visited: &FxHashSet<&'a Tile>) -> Vec<&Tile> {
        if visited.contains(self.get(x,y)) {
            return vec![];
        }
        
        let mut white: Vec<&Tile> = vec![self.get(x, y)];
        let mut black: Vec<&Tile> = vec![];
        
        while let Some(tile) = white.pop() {
            if !black.contains(&tile) {
                black.push(tile);
            }
            for (x, y) in self.adjacent(tile.x, tile.y) {
                let tile = self.get(x, y);
                if black.contains(&tile) || white.contains(&tile)
                    || self.pipes.iter().any(|t| t.x == tile.x && t.y == tile.y) {
                    continue;
                }
                white.push(tile);
            }
        }
        black
    }

}

impl Day<10> for Day10 {
    type Output = usize;
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

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY10_EXAMPLE1, 4),
                (DAY10_EXAMPLE2, 8),
                (DAY10_INPUT, 6768),
            ],
            test_cases![
                (DAY10_EXAMPLE3, 4),
                (DAY10_EXAMPLE3_1, 4),
                (DAY10_EXAMPLE4, 8),
                (DAY10_EXAMPLE5, 10),
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