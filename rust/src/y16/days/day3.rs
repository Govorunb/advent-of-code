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
        let parsed: Vec<Vec<usize>> = lines.map(|s| s.trim().split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect_vec()).collect_vec();
        match part {
            Part::One => {
                parsed.into_iter().filter(|t| test_triangle(t)).count()
            },
            Part::Two => {
                let cols = (0..3).map(|i| parsed.iter()
                    .map(move |l| l[i])
                    .collect_vec()
                ).flatten().collect_vec();
                // println!("{cols:?}");
                cols.into_iter()
                    .array_chunks::<3>()
                    .filter(|t| test_triangle(t))
                    .count()
            }
        }
    }
);

fn test_triangle(t: &[usize]) -> bool {
    // println!("Triangle {t:?}");
    let mut repeat = t.into_iter().cycle();
    for _ in 0..3 {
        let (a, b, c) = (repeat.next().unwrap(), repeat.next().unwrap(), repeat.next().unwrap());
        // println!("Examining ({a},{b},{c}); {a} {} {}", if *a >= b+c {">="} else {"<"}, b+c);
        if *a >= b+c {
            return false;
        }
        repeat.next();
    }
    true
}