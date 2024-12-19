use crate::*;

aoc_day!(
    day = 10,
    output = usize,
    examples = [],
    tests = [
        test_cases![
            (Self::INPUT, 360154),
        ],
        test_cases![
            (Self::INPUT, 5103798),
        ]
    ],
    solve = |input, part| {
        match part {
            Part::One => {
                let mut curr = input.to_string();
                for _ in 0..40 {
                    curr = Self::rle(curr);
                }
                curr.len()
            },
            Part::Two => {
                let mut curr = input.to_string();
                for _ in 0..50 {
                    curr = Self::rle(curr);
                }
                curr.len()
            }
        }
    }
);

#[derive(Debug, Clone, Copy, Default)]
struct Run(char, usize);

impl Day10 {
    fn rle(s: String) -> String {
        String::from_iter(
            s.chars()
                .rle()
                .map(|(c, count)| format!("{count}{c}"))
        )
    }
}