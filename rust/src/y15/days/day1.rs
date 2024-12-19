use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Day1;

impl Day<1> for Day1 {
    type Output = isize;
    const INPUT: &'static str = include_str!("../Input/day1.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        match part {
            Part::One => {
                let mut acc = 0;
                for char in input.chars() {
                    match char {
                        '(' => acc += 1,
                        ')' => acc -= 1,
                        _ => unreachable!()
                    }
                }
                acc
            },
            Part::Two => {
                let mut acc = 0;
                for (i, char) in input.char_indices() {
                    match char {
                        '(' => acc += 1,
                        ')' => acc -= 1,
                        _ => unreachable!()
                    }
                    if acc < 0 {
                        return i as isize + 1;
                    }
                }
                acc
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
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
            ],
            test_cases![
                (")", 1),
                ("()())", 5),
            ]
        ]
    }    
}
