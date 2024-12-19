use std::collections::HashMap;
use num::Integer;
use crate::*;

pub struct Day11;

impl Day<11> for Day11 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day11.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let blinks = match part {
            Part::One => 25,
            Part::Two => 75,
        };
        let mut stones = input.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .counts();
        for _ in 0..blinks {
            let curr_round = stones.clone();
            stones.clear();
            
            // borrowing rules made me put this here
            let mut update = |stone: usize, count|
                *stones.entry(stone).or_default() += count;
            
            for (stone, count) in curr_round.into_iter() {
                if stone == 0 {
                    update(1, count);
                } else {
                    let num_digits = 1 + stone.ilog10();
                    if num_digits % 2 == 0 {
                        let (left, right) = stone.div_rem(&10usize.pow(num_digits / 2));
                        update(left, count);
                        update(right, count);
                    } else {
                        update(stone * 2024, count);
                    }
                }
            }
            // println!("stones: {}", stones.values().sum::<usize>());
        }
        
        stones.values().sum()
    }
    const EXAMPLES: &'static [&'static str] = &[
        "125 17"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 55312),
                (Self::INPUT, 194557),
            ],
            test_cases![
                // (Self::EXAMPLES[0], 0),
                (Self::INPUT, 231532558973909),
            ]
        ]
    }
}


