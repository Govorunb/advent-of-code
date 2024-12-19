use crate::*;

aoc_day!(
    day = 4,
    output = u32,
    examples = [],
    tests = [
        test_cases![
            ("abcdef", 609043),
            ("pqrstuv", 1048970),
            (Self::INPUT, 282749),
        ],
        test_cases![
            // no examples for part 2 were provided
            (Self::INPUT, 9962624),
        ]
    ],
    solve = |input, part| {
        let mut input_md5_context = md5::Context::new();
        input_md5_context.consume(input);
        match part {
            Part::One => {
                (0..u32::MAX).into_par_iter()
                    .by_exponential_blocks()
                    .find_first(|i| {
                        let mut clone = input_md5_context.clone();
                        clone.consume(i.to_string());
                        let hash = clone.compute();
                        hash.starts_with(&[0, 0]) && hash[2] <= 0x0f
                    }).unwrap()
            },
            Part::Two => {
                (0..u32::MAX).into_par_iter()
                    .by_exponential_blocks()
                    .find_first(|i| {
                        let mut clone = input_md5_context.clone();
                        clone.consume(i.to_string());
                        let hash = clone.compute();
                        hash.starts_with(&[0, 0, 0])
                    }).unwrap()
            }
        }
    }
);
