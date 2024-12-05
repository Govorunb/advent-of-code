use crate::*;

pub struct Day10 {
    
}

impl Day<10> for Day10 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day10.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        match part {
            Part::One => {
                let mut curr = input.to_string();
                for _ in 0..40 {
                    curr = Self::rle(curr);
                }
                curr.len()
            },
            Part::Two => {
                let mut curr = input.to_string();
                for _ in 0..50 {
                    curr = Self::rle(curr);
                }
                curr.len()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (self.input(), 360154),
            ],
            test_cases![
                (self.input(), 5103798),
            ]
        ]
    }
}

impl Default for Day10 {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Run(char, usize);

impl Day10 {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn rle(s: String) -> String {
        let mut out = String::new();
        let mut curr_c: char = Default::default();
        let mut curr_count: usize = 0;
        for c in s.chars() {
            if c == curr_c {
                curr_count += 1;
            } else {
                if curr_count > 0 {
                    out.extend(curr_count.to_string().chars());
                    out.push(curr_c);
                }
                curr_c = c;
                curr_count = 1;
            }
        }
        out.extend(curr_count.to_string().chars());
        out.push(curr_c);
        
        out
    }
}