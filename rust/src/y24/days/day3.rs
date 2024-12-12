#![allow(dead_code)]
use crate::*;

pub struct Day3 {}

impl Day<3> for Day3 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day3.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        match part {
            Part::One => {
                let regex = Regex::new("mul\\((?<num1>\\d+),(?<num2>\\d+)\\)").unwrap();
                regex.captures_iter(input)
                    .map(Self::mul)
                    .sum::<usize>()
            },
            Part::Two => {
                let regex = Regex::new("(?<mul>mul\\((?<num1>\\d+),(?<num2>\\d+)\\))|(?<do_or_dont>do(?:n't)?\\(\\))").unwrap();
                let mut enabled = true;
                regex.captures_iter(input)
                    .map(|c|
                        if c.name("mul").is_some() {
                            if enabled { Self::mul(c) } else { 0 }
                        } else {
                            let do_or_dont = c.get(0).unwrap().as_str();
                            enabled = matches!(do_or_dont.chars().nth(2), Some('('));
                            0
                        }
                    ).sum::<usize>()
            },
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 161),
                (Self::INPUT, 179571322),
            ],
            test_cases![
                (Self::EXAMPLES[1], 48),
                (Self::INPUT, 103811193),
            ],
        ]
    }
}

impl Default for Day3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day3 {
    pub fn new() -> Self {
        Self {}
    }

    fn mul(c: Captures) -> usize {
        c.usize("num1") * c.usize("num2")
    }
}
