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
            (Self::INPUT, 176582889354075),
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
                let mut num = 0;
                let mut start = 0;
                for i in 0..batteries {
                    // can't have a 10 digit number that starts at the 5th from end
                    let curr = &row[start..=row.len()-batteries+i];
                    let max_pos = position_max_earliest(curr);
                    start += max_pos+1;
                    num = num * 10 + curr[max_pos];
                }
                num
            })
            .sum::<usize>()
    }
);

// position_max returns *latest* max element (e.g 90000009<---, presumably they check >= instead of just >)
fn position_max_earliest(source: &[usize]) -> usize {
    source.iter().enumerate().fold(0, |max_i, (i, &n)| {
        if n > source[max_i] { i } else { max_i }
    })
}
