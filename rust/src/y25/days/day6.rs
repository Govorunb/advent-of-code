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
                    let nums = nums.iter().map(|&n| n.bytes().usize());
                    match op {
                        "+" => nums.sum::<usize>(),
                        "*" => nums.product(),
                        _ => panic!("{op} instead of op"),
                    }
                }).sum::<usize>()
            },
            Part::Two => {
                p2_ltr(input)
            }
        }
    }
);

// very fast and "zero alloc"
fn p2_ltr(input: &str) -> usize {
    let mut total = 0;

    let grid = Grid::from_iter_2d_cringe(input.lines().map(|l| l.bytes()), None).unwrap();

    let mut acc = 1;
    let mut op = None;
    for x in 0..grid.width() {
        if op.is_none() {
            let c = grid[&(x, grid.height()-1).into()];
            if matches!(c, b'+' | b'*') {
                op = Some(c);
            }
        }
        let num = (0..grid.height()-1)
            .map(|y| grid[&(x, y).into()])
            .usize_filter();
        if num > 0 {
            match op {
                Some(b'+') => acc += num,
                Some(b'*') => acc *= num,
                _ => unreachable!(),
            }
        }
        if num == 0 || x == (grid.width()-1) {
            total += acc as usize;
            if let Some(b'+') = op {
                total -= 1;
            }
            acc = 1;
            op = None;
        }
    }
    total
}

// more readable and still fast (~2x slower)
fn p2_rtl(input: &str) -> usize {
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
