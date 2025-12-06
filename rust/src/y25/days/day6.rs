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
                let grid = Grid::from_iter_2d_cringe(
                    input.lines().map(|l| l.split_ascii_whitespace()),
                    None
                ).unwrap();
                grid.cols().map(|col| {
                    let (nums, op) = col.split_at(col.len()-1);
                    let op = op[0];
                    let nums = nums.iter().map(|n| n.parse::<usize>().unwrap());
                    match op {
                        "+" => nums.sum::<usize>(),
                        "*" => nums.product(),
                        _ => panic!("{op} instead of op"),
                    }
                }).sum::<usize>()
            },
            Part::Two => {
                let mut total = 0;

                let grid = Grid::from_iter_2d_cringe(input.lines().map(|l| l.chars()), None).unwrap();
                
                let mut nums: Vec<usize> = vec![];
                let mut num = String::with_capacity(grid.height());
                for x in (0..grid.width()).rev() {
                    num.clear();
                    num.extend(
                        (0..grid.height())
                        .filter_map(|y| grid.get(&(x, y).into()))
                        .filter(|c| c.is_ascii_digit())
                    );
                    if !num.is_empty() {
                        nums.push(num.parse().unwrap());
                        let op = grid.get(&(x, grid.height()-1).into()).unwrap();
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
