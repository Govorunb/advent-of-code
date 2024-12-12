use crate::*;

pub struct Day9 {
    
}

struct History {
    sequences: Vec<Sequence>,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct Sequence {
    values: Vec<isize>,
}

impl From<&str> for Sequence {
    fn from(s: &str) -> Self {
        let values = s
            .split_ascii_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect_vec();
        Self { values }
    }
}

impl Sequence {
    fn differences(&self) -> Self {
        let values = self.values.iter()
            .tuple_windows()
            .map(|(a,b)| b-a)
            .collect_vec();
        Self { values }
    }
}

impl History {
    fn predict_next(&self) -> isize {
        self.sequences.iter().rev()
            .map(|seq| seq.values.last().unwrap())
            .sum()
    }
    fn predict_prev(&self) -> isize {
        self.sequences.iter().rev()
            .map(|seq| seq.values.first().unwrap())
            .fold(0, |acc, val| val - acc)
    }
    fn fill(&mut self) {
        let mut curr_seq = self.sequences[0].clone();
        while curr_seq.values.iter().any(|&v| v != 0) {
            let new_seq = curr_seq.differences();
            self.sequences.push(new_seq.clone());
            curr_seq = new_seq;
        }
        self.sequences.pop();
    }
}

impl Day<9> for Day9 {
    type Output = isize;
    const INPUT: &'static str = include_str!("../Input/day9.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        input.lines()
            .map(|l| {
                let mut hist = History {sequences: vec![l.into()]};
                hist.fill();
                match part {
                    Part::One => hist.predict_next(),
                    Part::Two => hist.predict_prev(),
                }
            })
            .sum()
    }
    const EXAMPLES: &'static [&'static str] = &[
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 114),
                (Self::INPUT, 2101499000),
            ],
            test_cases![
                (Self::EXAMPLES[0], 2),
                (Self::INPUT, 1089),
            ]
        ]
    }
}

impl Default for Day9 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day9 {
    pub fn new() -> Self {
        Self {
        }
    }
}