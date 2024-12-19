// replace all 20 with the day number
#![allow(dead_code)]
use crate::*;

pub struct Day20;

impl Day<20> for Day20 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day20.txt");

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
    const EXAMPLES: &'static [&'static str] = &[
        ""
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 0),
                // (Self::INPUT, 0),
            ],
            test_cases![
                // (Self::EXAMPLES[0], 0),
                // (Self::INPUT, 0),
            ]
        ]
    }
}


