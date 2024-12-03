// replace all 20 with the day number
#![allow(dead_code)]
use crate::test_cases;
use crate::common::*;

pub const DAY20_INPUT: &str = include_str!("../Input/day20.txt");
pub const DAY20_EXAMPLE: &str =
"";

pub struct Day20 {
    
}

impl Day<20> for Day20 {
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
                (DAY20_EXAMPLE, 0),
                // (DAY20_INPUT, 0),
            ],
            test_cases![
                // (DAY20_EXAMPLE, 0),
                // (DAY20_INPUT, 0),
            ]
        ]
    }
}

impl Default for Day20 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day20 {
    pub fn new() -> Self {
        Self {
        }
    }
}