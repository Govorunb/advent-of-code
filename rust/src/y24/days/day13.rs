use crate::*;

pub struct Day13 {
    
}

impl Day<13> for Day13 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day13.txt");
    const EXAMPLES: &'static [&'static str] = &[
""
];
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

impl Default for Day13 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day13 {
    pub fn new() -> Self {
        Self {
        }
    }
}