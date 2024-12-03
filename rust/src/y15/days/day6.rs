use crate::test_cases;
use crate::common::*;

const DAY6_EXAMPLE: &str =
"";

pub struct Day6 {
    
}

impl Day<6> for Day6 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day6.txt");
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
                (DAY6_EXAMPLE, 0),
                // (DAY6_INPUT, 0),
            ],
            test_cases![
                // (DAY6_EXAMPLE, 0),
                // (DAY6_INPUT, 0),
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
}