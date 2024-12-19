use crate::*;

aoc_day!(
    day = 3,
    output = usize,
    examples = [
"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 161),
            (Self::INPUT, 179571322),
        ],
        test_cases![
            (Self::EXAMPLES[1], 48),
            (Self::INPUT, 103811193),
        ],
    ],
    solve = |input, part| {
        let mul = |c: Captures| {c.usize("num1") * c.usize("num2")};
        match part {
            Part::One => {
                let regex = Regex::new("mul\\((?<num1>\\d+),(?<num2>\\d+)\\)").unwrap();
                regex.captures_iter(input)
                    .map(mul)
                    .sum::<usize>()
            },
            Part::Two => {
                let regex = Regex::new("(?<mul>mul\\((?<num1>\\d+),(?<num2>\\d+)\\))|(?<do_or_dont>do(?:n't)?\\(\\))").unwrap();
                let mut enabled = true;
                regex.captures_iter(input)
                    .map(|c|
                        if c.name("mul").is_some() {
                            if enabled { mul(c) } else { 0 }
                        } else {
                            let do_or_dont = c.get(0).unwrap().as_str();
                            enabled = matches!(do_or_dont.chars().nth(2), Some('('));
                            0
                        }
                    ).sum::<usize>()
            },
        }
    }
);
