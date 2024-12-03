#![allow(dead_code)]
use super::common::*;
use crate::days::*;

const INPUT: &'static str = include_str!("../Input/day1.txt");
pub const DAY1_EXAMPLE: &str =
"";

pub struct Day1 {
    
}

impl Day<1> for Day1 {
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
                (DAY1_EXAMPLE, 0),
                // (self.input(), 0),
            ],
            test_cases![
                // (DAY1_EXAMPLE, 0),
                // (self.input(), 0),
            ]
        ]
    }
}

impl Default for Day1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day1 {
    pub fn new() -> Self {
        Self {
        }
    }
}