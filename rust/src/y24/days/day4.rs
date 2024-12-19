use std::iter;
use std::ops::Index;
use std::str::FromStr;
use std::sync::LazyLock;
use crate::*;

pub struct Day4;

impl Day<4> for Day4 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day4.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let grid: Grid<char> = Grid::from_str(input).unwrap();
        match part {
            Part::One => {
                Self::word_search(grid,"XMAS", &Vector2::AROUND)
            },
            Part::Two => {
                grid.coords()
                    .filter(|pt| {
                        grid[pt] == 'A' // centered on 'A'
                        && Vector2::CORNERS.iter().take(2)
                            .filter(|&dir| {
                                let pt2 = pt + dir; // top left/right
                                let opp = pt - dir; // bottom right/left
                                matches!(
                                    (grid.get(&pt2), grid.get(&opp)),
                                    (Some('M'), Some('S')) | (Some('S'), Some('M'))
                                )
                            }).count() == 2
                    }).count()
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"..X...
.SAMX.
.A..A.
XMAS.S
.X....",
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 4),
                (Self::EXAMPLES[1], 18),
                (Self::INPUT, 2557),
            ],
            test_cases![
                (Self::EXAMPLES[1], 9),
                (Self::INPUT, 1854),
            ]
        ]
    }
}

impl Day4 {
    
    fn word_search<'a>(grid: Grid<char>, pat: &str, directions: &[Vector2]) -> usize {
        let mut chars = pat.chars();
        let head = chars.next().unwrap();
        let tail: &Vec<char> = &chars.collect_vec();
        grid.coords()
            .filter(|pt| grid[pt] == head) // e.g. "ABCD" can only start from 'A'
            .cartesian_product(directions) // search all cells around
            .filter(|&(pt, &dir)|
                grid.ray(pt, dir).skip(1) // skip pt itself
                    .map(|(_p,s)| s)
                    .take(tail.len())
                    .eq(tail)
            ).count()
        // imperative version:
        /*
        let mut total = 0;
        for (pt, &c) in grid.cells() {
            if c != head {continue}
        
            for dir in Direction8::iter() {
                let mut curr = pt.clone();
                let found_word = tail.iter().all(|&test| {
                    curr = &curr + dir;
                    grid.get(&curr).is_some_and(|&x| x == test)
                });
                if found_word { total += 1 }
            }
        }
        total
        */
    }
}