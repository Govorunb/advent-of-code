use crate::test_cases;
use crate::common::*;

pub const DAY18_INPUT: &str = include_str!("../Input/day18.txt");
pub const DAY18_EXAMPLE: &str =
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

pub struct Day18 {
    
}

struct Instruction {
    dir: Direction,
    length: usize,
    color: usize,
}

impl Day<18> for Day18 {
    type Output = usize;
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        match part {
            Part::One => {
                lines.count()
            },
            Part::Two => {
                0
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY18_EXAMPLE, 62),
                // (DAY18_INPUT, 0),
            ],
            test_cases![
                // (DAY18_EXAMPLE, 0),
                // (DAY18_INPUT, 0),
            ]
        ]
    }
}

impl Default for Day18 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day18 {
    pub fn new() -> Self {
        Self {
        }
    }
}