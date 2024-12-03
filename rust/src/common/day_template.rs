// replace all ~ with the day number
use crate::*;

pub const DAY~_EXAMPLE: &str =
"";

pub struct Day~ {
    
}

impl Day<~> for Day~ {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day~.txt");
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
                (DAY~_EXAMPLE, 0),
                // (self.input(), 0),
            ],
            test_cases![
                // (DAY~_EXAMPLE, 0),
                // (self.input(), 0),
            ]
        ]
    }
}

impl Default for Day~ {
    fn default() -> Self {
        Self::new()
    }
}

impl Day~ {
    pub fn new() -> Self {
        Self {
        }
    }
}