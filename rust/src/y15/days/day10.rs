use crate::*;

pub struct Day10;

impl Day<10> for Day10 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day10.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
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

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::INPUT, 360154),
            ],
            test_cases![
                (Self::INPUT, 5103798),
            ]
        ]
    }
}

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