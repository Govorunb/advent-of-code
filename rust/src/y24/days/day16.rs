use crate::*;

pub struct Day16 {
    
}

impl Day<16> for Day16 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day16.txt");
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