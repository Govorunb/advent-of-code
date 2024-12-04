#![allow(dead_code)]
use crate::*;

pub const DAY3_EXAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
pub const DAY3_EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

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

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY3_EXAMPLE1, 161),
                (self.input(), 179571322),
            ],
            test_cases![
                (DAY3_EXAMPLE2, 48),
                (self.input(), 103811193),
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
        c.name("num1").unwrap().as_str().parse::<usize>().unwrap()
        * c.name("num2").unwrap().as_str().parse::<usize>().unwrap()
    }
}
