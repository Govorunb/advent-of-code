use crate::*;

pub struct Day1 {}

impl Day<1> for Day1 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day1.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        match part {
            Part::One => {
                lines.count()
            },
            Part::Two => {
                0
            },
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
            ],
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
        Self {}
    }
}
