use crate::*;

aoc_day!(
    day = 23,
    output = usize,
    examples = [""],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 0),
            // (Self::INPUT, 0),
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
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
);
