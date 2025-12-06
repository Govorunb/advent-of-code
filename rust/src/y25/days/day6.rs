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
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines().map(|l| l.split_ascii_whitespace().collect_vec()).collect_vec();
        match part {
            Part::One => {
                let nums = lines.iter().take(lines.len()-1).map(|l| l.into_iter().map(|n| n.parse::<usize>().expect(n)).collect_vec()).collect_vec();
                let ops = lines.iter().last().unwrap().into_iter().map(|c| c.parse::<Op>().expect(c)).collect_vec();
                let width = nums[0].len();
                let mut total = 0;
                for y in 0..width {
                    let operands = nums.iter().map(|x| x[y]).collect_vec();
                    let op = ops[y];
                    total += match op {
                        Op::Add => operands.iter().sum::<usize>(),
                        Op::Mult => operands.iter().product(),
                    };
                }
                total
            },
            Part::Two => {
                let mut total = 0;
                
                let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
                let width = lines[0].len();
                
                let mut nums: Vec<usize> = vec![];
                let mut col = String::with_capacity(lines.len());
                let mut op = Op::Add;
                for y in (0..width).rev() {
                    col.clear();
                    col.extend(lines.iter().map(|l| l[y]));
                    let mut s = col.as_str();
                    if matches!(s.chars().last().unwrap(), '*' | '+') {
                        op = s.chars().last().unwrap().into();
                        s = &s[..(s.len()-1)];
                    }
                    s = s.trim_ascii();
                    if s.is_empty() {
                        total += match op {
                            Op::Add => nums.drain(..).sum::<usize>(),
                            Op::Mult => nums.drain(..).product(),
                        };
                    } else {
                        nums.push(s.parse().expect(s));
                    }
                }
                total += match op {
                    Op::Add => nums.drain(..).sum::<usize>(),
                    Op::Mult => nums.drain(..).product(),
                };
                total
            }
        }
    }
);

#[derive(Clone, Copy, Debug)]
enum Op { Add, Mult }

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mult),
            _ => Err(())
        }
    }
}

impl From<char> for Op {
    fn from(value: char) -> Self {
        match value {
            '+' => Op::Add,
            '*' => Op::Mult,
            _ => unreachable!(),
        }
    }
}