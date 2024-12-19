use crate::*;

use crate::*;

aoc_day!(
    day = 1,
    output = isize,
    examples = [],
    tests = [
        test_cases![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
            (Self::INPUT, 138),
        ],
        test_cases![
            (")", 1),
            ("()())", 5),
            (Self::INPUT, 1771),
        ]
    ],
    solve = |input, part| {
        let value = |c| if c == '(' { 1 } else { -1 };
        match part {
            Part::One => {
                input.chars()
                    .map(value)
                    .sum()
            },
            Part::Two => {
                let mut acc = 0;
                input.chars().find_position(|&c| {
                    acc += value(c);
                    acc < 0
                }).map(|(i, _)| i).unwrap() as isize + 1
            }
        }
    }
);
