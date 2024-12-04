use std::str::FromStr;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::test_cases;
use crate::common::*;

pub const DAY16_EXAMPLE: &str =
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

pub const DAY16_TEST_1: &str = 
r".|......
.-\.....
..../\..
.|../...
..\../..";

pub const DAY16_TEST_2: &str = 
r".|.....\
.-\.....
..../\..
.|..\../
..\../..";

pub const DAY16_TEST_3: &str = 
r"|..........
...........
-\.........";

pub struct Day16 {
    
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Contraption {
    grid: Grid<Cell>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    id: usize,
    x: usize,
    y: usize,
    dir: Direction,
}

impl FromStr for Contraption {
    type Err = <Grid<Cell> as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: s.parse()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
enum Cell {
    #[default]
    Empty, // .
    Mirror(Mirror), // / or \
    Splitter(Splitter), // | or -
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Mirror {
    Slash, // /
    Backslash, // \
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Splitter {
    Vertical, // |
    Horizontal, // -
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum CellEnergy {
    #[default]
    Empty,
    Energized
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '/' => Cell::Mirror(Mirror::Slash),
            '\\' => Cell::Mirror(Mirror::Backslash),
            '|' => Cell::Splitter(Splitter::Vertical),
            '-' => Cell::Splitter(Splitter::Horizontal),
            _ => unreachable!()
        }
    }
}

impl Mirror {
    fn reflect(&self, dir: Direction) -> Direction {
        match (self, dir) {
            (Mirror::Slash, Direction::North) => Direction::East,
            (Mirror::Slash, Direction::South) => Direction::West,
            (Mirror::Slash, Direction::East) => Direction::North,
            (Mirror::Slash, Direction::West) => Direction::South,
            (Mirror::Backslash, Direction::North) => Direction::West,
            (Mirror::Backslash, Direction::South) => Direction::East,
            (Mirror::Backslash, Direction::East) => Direction::South,
            (Mirror::Backslash, Direction::West) => Direction::North,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Empty => '.',
            Cell::Mirror(Mirror::Slash) => '/',
            Cell::Mirror(Mirror::Backslash) => '\\',
            Cell::Splitter(Splitter::Vertical) => '|',
            Cell::Splitter(Splitter::Horizontal) => '-',
        })?;
        Ok(())
    }
}

impl Display for CellEnergy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CellEnergy::Empty => '.',
            CellEnergy::Energized => '#',
        })?;
        Ok(())
    }
}

impl From<bool> for CellEnergy {
    fn from(b: bool) -> Self {
        if b { CellEnergy::Energized } else { CellEnergy::Empty }
    }
}

impl Contraption {
    fn fire_beam(&self, x: usize, y: usize, dir: Direction) -> Grid<u8> {
        // bitfield for the 4 directions - this way we can track looping beams
        let mut grid_energy: Grid<u8> = Grid::from_origin(self.grid.size()).unwrap();
        let mut initial_beam = Beam {id: 0, x, y, dir};
        Self::step_beam(&mut initial_beam, &self.grid[Point::from((x,y))]); // totally not a hack
        let mut beams: Vec<Beam> = vec![initial_beam];
        let mut top_id = 1;
        while !beams.is_empty() {
            let mut new_beams = vec![];
            beams.retain_mut(|beam| {
                // the order of operations in the loop is:
                // die -> energize -> move -> turn
                let cell_energized = grid_energy.get_mut(&Point::from((beam.x, beam.y))).unwrap();
                
                // die
                let mask = 1 << (beam.dir as u8);
                let already_been_here = *cell_energized & mask > 0;
                if already_been_here {
                    // beam did not move to a new cell last iteration
                    // this means it's (a) moving past the boundaries, or (b) looping
                    // in both cases, it will never energize any more cells
                    return false;
                }

                // energize
                *cell_energized |= mask;

                // move
                match beam.dir.move_(beam.x, beam.y) {
                    Some((x,y)) if x < self.grid.width() && y < self.grid.height() => {
                        beam.x = x;
                        beam.y = y;
                    },
                    // can't move past the bounds => die
                    _ => return false,
                }

                // turn
                let cell = &self.grid[Point::from((beam.x, beam.y))];
                let split = Self::step_beam(beam, cell);
                if split {
                    let new_beam = Beam {
                        id: top_id,
                        x: beam.x,
                        y: beam.y,
                        dir: beam.dir.opp(),
                    };
                    new_beams.push(new_beam);
                    top_id += 1;
                }

                true
            });
            beams.append(&mut new_beams);
        }
        grid_energy
    }

    fn step_beam(beam: &mut Beam, cell: &Cell) -> bool {
        let (dir, split) = match cell {
            Cell::Mirror(mirror) => {
                (mirror.reflect(beam.dir), false)
            },
            Cell::Splitter(Splitter::Vertical) if matches!(beam.dir, Direction::West | Direction::East) => {
                // doesn't matter which direction so just pick one
                (Direction::South, true)
            },
            Cell::Splitter(Splitter::Horizontal) if matches!(beam.dir, Direction::North | Direction::South) => {
                (Direction::East, true)
            }
            // in all other cases
            // (empty, V splitter when going north/south, H splitter going west/east)
            // move unimpeded
            _ => (beam.dir, false),
        };
        beam.dir = dir;
        split
    }

    fn count_energized(&self, x: usize, y: usize, dir: Direction) -> usize {
        self.fire_beam(x, y, dir)
            .par_elements()
            .filter(|&&cell| cell != 0)
            .count()
    }
}


impl Day<16> for Day16 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day16.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let c: Contraption = input.parse().expect("failed to parse grid");
        match part {
            Part::One => {
                c.count_energized(0, 0, Direction::East)
            },
            Part::Two => {
                let width = c.grid.width();
                let height = c.grid.height();
                // // functional
                // (0..height)
                //     .flat_map(|y| [
                //         c.count_energized(0, y, Direction::East),
                //         c.count_energized(width-1, y, Direction::West)
                //     ])
                // .max().unwrap()
                // .max((0..width)
                //     .flat_map(|x| [
                //         c.count_energized(x, 0, Direction::South),
                //         c.count_energized(x, height-1, Direction::North)
                //     ])
                //     .max().unwrap())

                // // imperative
                // let mut max = 0;
                // for y in 0..height {
                //     max = max
                //         .max(c.count_energized(0, y, Direction::East))
                //         .max(c.count_energized(width-1, y, Direction::West));
                // }
                // for x in 0..width {
                //     max = max
                //         .max(c.count_energized(x, 0, Direction::South))
                //         .max(c.count_energized(x, height-1, Direction::North));
                // }
                // max

                let vertical = (0..height)
                    .into_par_iter()
                    .flat_map_iter(|y| [
                        c.count_energized(0, y, Direction::East),
                        c.count_energized(width-1, y, Direction::West)
                    ]); // if we put .max().unwrap() here, we're forced to wait for vertical before we can start horizontal

                let horizontal = (0..width)
                    .into_par_iter()
                    .flat_map_iter(|x| [
                        c.count_energized(x, 0, Direction::South),
                        c.count_energized(x, height-1, Direction::North)
                    ]);
                // start both running parallel
                let (max_vert, max_horiz) = rayon::join(|| vertical.max().unwrap(), || horizontal.max().unwrap());
                
                max_vert.max(max_horiz)
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY16_EXAMPLE, 46),
                (DAY16_TEST_1, 19),
                (DAY16_TEST_2, 25),
                (DAY16_TEST_3, 4),
                (self.input(), 7517),
            ],
            test_cases![
                (DAY16_EXAMPLE, 51),
                (self.input(), 7741),
            ]
        ]
    }
}

impl Default for Day16 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day16 {
    pub fn new() -> Self {
        Self {
        }
    }
}