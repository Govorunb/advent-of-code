use std::str::FromStr;
use std::sync::LazyLock;
use crate::*;

pub const DAY4_EXAMPLE2: &str =
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
pub const DAY4_EXAMPLE1: &str = 
"..X...
.SAMX.
.A..A.
XMAS.S
.X....";

pub struct Day4;

impl Day<4> for Day4 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day4.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let grid: Grid<char> = Grid::from_str(input).unwrap();
        match part {
            Part::One => {
                let mut total = 0;
                for (pt, &c) in grid.cells() {
                    if c != 'X' {continue}
                    
                    for dir in Direction8::iter() {
                        let mut curr = pt.clone();
                        let xmas = ['M', 'A', 'S'].iter().all(|&test| {
                            curr = &curr + dir;
                            grid.get(&curr).is_some_and(|&x| x == test)
                        });
                        if xmas { total += 1 }
                    }
                }
                
                total
            },
            Part::Two => {
                let mut total = 0;
                for (pt, &c) in grid.cells() {
                    if c != 'A' {continue}
                    
                    let mut count_mas = 0;
                    let top_corners = Direction8::corners().take(2);
                    for dir in top_corners {
                        let pt2 = &pt + &dir;
                        let opp = &pt - &dir;
                        match (grid.get(&pt2), grid.get(&opp)) {
                            (Some('M'), Some('S'))
                            | (Some('S'), Some('M')) => count_mas += 1,
                            _ => break,
                        };
                    }
                    if count_mas == 2 {
                        total += 1
                    }
                }

                total
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY4_EXAMPLE1, 4),
                (DAY4_EXAMPLE2, 18),
                (self.input(), 2557),
            ],
            test_cases![
                (DAY4_EXAMPLE2, 9),
                (self.input(), 1854),
            ]
        ]
    }
}

impl Default for Day4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day4 {
    pub fn new() -> Self {
        Self {
        }
    }
}