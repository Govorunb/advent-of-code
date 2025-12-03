use crate::*;

aoc_day!(
    day = 3,
    output = usize,
    examples = [
"987654321111111
811111111111119
234234234234278
818181911112111",
    ],
    tests = [
        test_cases![
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
            (Self::EXAMPLES[0], 357),
            (Self::INPUT, 17766),
        ],
        test_cases![
            ("987654321111111", 987654321111),
            ("811111111111119", 811111111119),
            ("234234234234278", 434234234278),
            ("818181911112111", 888911112111),
            (Self::EXAMPLES[0], 3121910778619),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let grid = Grid::from_digits(input, 10);
        let batteries = match part {
            Part::One => 2,
            Part::Two => 12
        };
        grid.rows()
            .map(|row| {
                let mut total = 0;
                let mut left_pos = 0;
                for i in 0..batteries {
                    let curr = &row[left_pos..=row.len()-batteries+i];
                    let max_pos = max_left_pos(curr);
                    left_pos += max_pos+1;
                    total = total * 10 + curr[max_pos];
                }
                total
            })
            .sum::<usize>()
    }
);

fn max_left_pos(row: &[usize]) -> usize {
    row.iter().enumerate()
        .fold(0, |acc, (i, &e)| {
        if e > row[acc] { i } else {acc}
    })
}