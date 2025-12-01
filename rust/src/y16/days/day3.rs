use crate::*;

aoc_day!(
    day = 3,
    output = usize,
    examples = [
        "5 10 25",
"2 3 4
4 5 6
99999 1 1
",
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 0),
            (Self::EXAMPLES[1], 2),
            (Self::INPUT, 869),
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            (Self::INPUT, 1544),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        let parsed: Vec<(usize, usize, usize)> = lines.map(|s|
            s.trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .tuples::<(_,_,_)>()
                .next().unwrap()
        ).collect_vec();
        match part {
            Part::One => {
                parsed.into_iter().filter(|&(a,b,c)| test_triangle(&[a,b,c])).count()
            },
            Part::Two => {
                // transpose/multiunzip/whatever you wanna call it
                let (a,b,c): (Vec<_>, Vec<_>, Vec<_>) = parsed.into_iter().collect();
                [a,b,c].into_iter()
                    .flatten()
                    .array_chunks::<3>()
                    .filter(test_triangle)
                    .count()
            }
        }
    }
);

fn test_triangle(t: &[usize; 3]) -> bool {
    let [a,b,c] = *t;
    a < b+c
    && b < a+c
    && c < a+b
}