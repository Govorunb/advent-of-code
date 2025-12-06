use std::str::FromStr;

use crate::*;

aoc_day!(
    day = 6,
    output = usize,
    examples = [
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 4277556),
            (Self::INPUT, 6957525317641),
        ],
        test_cases![
            (Self::EXAMPLES[0], 3263827),
            (Self::INPUT, 13215665360076),
        ]
    ],
    solve = |input, part| {
        match part {
            Part::One => {
                let lines = input.lines().map(|l| l.split_ascii_whitespace()).collect_vec();
                let mut buf = Vec::with_capacity(lines.len());
                let cols = transpose(lines.into_iter());
                cols.map(|col| {
                    buf.extend(col);
                    let op = buf.pop().unwrap();
                    let nums = buf.drain(..).map(|n| n.parse::<usize>().unwrap());
                    match op {
                        "+" => nums.sum::<usize>(),
                        "*" => nums.product(),
                        _ => panic!("{op} instead of op"),
                    }
                }).sum::<usize>()
            },
            Part::Two => {
                let mut total = 0;
                
                let cols = transpose(input.lines().map(|l| l.chars()))
                    .map(|c| c.collect_vec())
                    .collect_vec();
                
                let mut nums: Vec<usize> = vec![];
                let mut num = String::with_capacity(cols[0].len());
                for col in cols.iter().rev() {
                    num.clear();
                    num.extend(col.iter().filter(|c| c.is_ascii_digit()));
                    if !num.is_empty() {
                        nums.push(num.parse().unwrap());
                        let op = col.last().unwrap();
                        total += match op {
                            '+' => nums.drain(..).sum::<usize>(),
                            '*' => nums.drain(..).product(),
                            ' ' => 0,
                            _ => unreachable!(),
                        };
                    }
                }
                total
            }
        }
    }
);
