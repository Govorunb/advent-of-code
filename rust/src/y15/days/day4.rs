use crate::*;

pub struct Day4;

impl Day<4> for Day4 {
    type Output = u32;
    const INPUT: &'static str = include_str!("../Input/day4.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let mut input_md5_context = md5::Context::new();
        input_md5_context.consume(input);
        match part {
            Part::One => {
                (0..u32::MAX).into_par_iter()
                    .find_first(|i| {
                        let mut clone = input_md5_context.clone();
                        clone.consume(i.to_string());
                        let hash = clone.compute();
                        hash.starts_with(&[0, 0]) && hash[2] <= 0x0f
                    }).unwrap()
            },
            Part::Two => {
                (0..u32::MAX).into_par_iter()
                    .find_first(|i| {
                        let mut clone = input_md5_context.clone();
                        clone.consume(i.to_string());
                        let hash = clone.compute();
                        hash.starts_with(&[0, 0, 0])
                    }).unwrap()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                ("abcdef", 609043),
                ("pqrstuv", 1048970),
                (Self::INPUT, 282749),
            ],
            test_cases![
                // no examples for part 2 were provided
                (Self::INPUT, 9962624),
            ]
        ]
    }
}

