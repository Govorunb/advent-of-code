use std::fmt::Display;
use crate::*;

aoc_day!(
    day = 14,
    output = usize,
    examples = [
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 136),
            (Self::INPUT, 109424),
        ],
        test_cases![
            (Self::EXAMPLES[0], 64),
            (Self::INPUT, 102509),
        ]
    ],
    solve = |input, part| {
        let mut grid = Platform::parse(input);
        match part {
            Part::One => {
                grid.roll_north();
            },
            Part::Two => {
                grid.spin(1_000_000_000);
            },
        }
        grid.load()
    }
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    grid: Grid<Cell>, 
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Cell {
    Empty,
    Ball,
    Cube,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'O' => Self::Ball,
            '#' => Self::Cube,
            _ => unreachable!("wrong char {c}"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Empty => '.',
            Cell::Ball => 'O',
            Cell::Cube => '#',
        })?;
        Ok(())
    }
}

impl Platform {
    fn parse(s: &str) -> Self {
        Self { grid: s.parse().expect("failed to parse grid") }
    }

    fn height(&self) -> usize {
        self.grid.height()
    }

    fn width(&self) -> usize {
        self.grid.width()
    }

    fn load(&self) -> usize {
        self.grid.cols()
            .map(|col| col.iter()
                .enumerate()
                .filter_map(|(y, c)| if let Cell::Ball = c { Some(self.height() - y) } else { None })
                .sum::<usize>()
        ).sum()
    }

    fn spin(&mut self, n: usize) {
        let mut seen: FxIndexSet<Platform> = FxIndexSet::default();
        for i in 0..n {
            seen.insert(self.clone());
            self.cycle();
            let cycle = i+1;
            // println!("After {cycle} cycles:");
            // println!("{self}");
            // println!("load {}", self.load()); // this really helped in debugging cycle finding

            if let Some(h_i) = seen.get_index_of(self) {
                let period = cycle - h_i;
                let left = (n-cycle) % period;
                // fast-forward since we know the target state is in the set
                let end_i = h_i+left;
                // println!("found loop at cycle {cycle} - matches {h_i}, period {period}");
                // println!("{left} left");
                // println!("fast-forwarding to {end_i}");
                self.grid = seen.get_index(end_i).unwrap().grid.clone();
                // println!("Final state:");
                // println!("{self}");
                break;
            }
        }
    }

    fn cycle(&mut self) {
        //println!("Rolling north:");
        self.roll_north();
        //println!("{self}");
        //println!("Rolling west:");
        self.roll_west();
        //println!("{self}");
        //println!("Rolling south:");
        self.roll_south();
        //println!("{self}");
        //println!("Rolling east:");
        self.roll_east();
    }

    fn roll_north(&mut self) {
        let height = self.height();
        for mut col in self.grid.cols_mut() {
            let mut top_empty = 0;
            for y in 0..height {
                match col[y] {
                    Cell::Cube => {
                        top_empty = y+1;
                    },
                    Cell::Ball => {
                        if top_empty < y {
                            col[y] = Cell::Empty;
                            col[top_empty] = Cell::Ball;
                        }
                        top_empty += 1;
                    },
                    Cell::Empty => {},
                }
            }
        }
    }
    fn roll_south(&mut self) {
        let height = self.height();
        for mut col in self.grid.cols_mut() {
            let mut bottom_empty = height-1;
            for y in (0..height).rev() {
                match col[y] {
                    Cell::Cube => {
                        // new bottom
                        // doesn't matter if we wrap at 0 because we're done iterating
                        bottom_empty = y.wrapping_sub(1);
                    },
                    Cell::Ball => {
                        // roll down
                        if bottom_empty > y {
                            col[y] = Cell::Empty;
                            col[bottom_empty] = Cell::Ball;
                        }
                        // same thing with setting it to 0 as in the Cube case
                        bottom_empty = bottom_empty.wrapping_sub(1);
                    },
                    Cell::Empty => {},
                }
            }
        }
    }
    fn roll_west(&mut self) {
        let width = self.width();
        for row in self.grid.rows_mut() {
            let mut left_empty = 0;
            for x in 0..width {
                match row[x] {
                    Cell::Cube => {
                        // new edge
                        left_empty = x+1;
                    },
                    Cell::Ball => {
                        // roll left
                        if left_empty < x {
                            row[x] = Cell::Empty;
                            row[left_empty] = Cell::Ball;
                        }
                        left_empty += 1;
                    },
                    Cell::Empty => {},
                }
            }
        }
    }
    fn roll_east(&mut self) {
        let width = self.width();
        for row in self.grid.rows_mut() {
            let mut right_empty = width-1;
            for x in (0..width).rev() {
                match row[x] {
                    Cell::Cube => {
                        // new edge
                        // see comment in roll_south for why we don't care about it wrapping
                        right_empty = x.wrapping_sub(1);
                    },
                    Cell::Ball => {
                        // roll left
                        if right_empty > x {
                            row[x] = Cell::Empty;
                            row[right_empty] = Cell::Ball;
                        }
                        right_empty = right_empty.wrapping_sub(1);
                    },
                    Cell::Empty => {},
                }
            }
        }
    }
}
