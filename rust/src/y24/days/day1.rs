#![allow(dead_code)]

use crate::test_cases;
use crate::common::*;

pub const DAY1_INPUT: &str = include_str!("../Input/day1.txt");
pub const DAY1_EXAMPLE: &str =
"3   4
4   3
2   5
1   3
3   9
3   3";

pub struct Day1 {
}

impl Day<1> for Day1 {
    type Output = usize;
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        let (mut left, mut right): (Vec<_>, Vec<_>) = lines
            .map(|l| l.split_once("   ").unwrap())
            .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
            .unzip();
        match part {
            Part::One => {
                left.sort_unstable();
                right.sort_unstable();
                (0..left.len())
                    .map(|i| left[i].abs_diff(right[i]))
                    .sum()
            },
            Part::Two => {
                let max = *right.iter().max().unwrap();
                let mut right_occ: Vec<usize> = vec![0; max + 1];
                for num in right {
                    right_occ[num] += 1;
                }
                
                left.into_iter()
                    .map(|n| n * right_occ[n])
                    .sum()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY1_EXAMPLE, 11),
                (DAY1_INPUT, 2375403),
            ],
            test_cases![
                (DAY1_EXAMPLE, 31),
                // (DAY1_INPUT, 0),
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