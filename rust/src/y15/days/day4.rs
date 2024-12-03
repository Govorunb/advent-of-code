use crate::test_cases;
use crate::common::*;

pub const DAY4_INPUT: &str = include_str!("../Input/day4.txt");

pub struct Day4 {
}

impl Day<4> for Day4 {
    type Output = u32;
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let mut input_md5_context = md5::Context::new();
        input_md5_context.consume(input);
        match part {
            Part::One => {
                for i in 0..u32::MAX {
                    let mut clone = input_md5_context.clone();
                    clone.consume(i.to_string());
                    let hash = clone.compute();
                    if !hash.starts_with(&[0, 0]) || hash[2] > 0x0f {
                        continue;
                    }
                    return i;
                }
                unreachable!("not found")
            },
            Part::Two => {
                for i in 0..u32::MAX {
                    let mut clone = input_md5_context.clone();
                    clone.consume(i.to_string());
                    let hash = clone.compute();
                    if hash.starts_with(&[0, 0, 0]) {
                        return i
                    }
                }
                unreachable!("not found")
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                ("abcdef", 609043),
                ("pqrstuv", 1048970),
            ],
            test_cases![
                // no tests for part 2 were provided
            ]
        ]
    }
}

impl Default for Day4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day4 {
    pub fn new() -> Self {
        Self {
        }
    }
}