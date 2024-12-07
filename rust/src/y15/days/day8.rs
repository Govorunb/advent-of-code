use crate::*;

pub const DAY8_EXAMPLE: &str =
r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;

pub struct Day8;

impl Day<8> for Day8 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day8.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        match part {
            Part::One => {
                lines
                    .map(Self::decode_size_diff)
                    .sum()
            },
            Part::Two => {
                lines
                    .map(Self::encode_size_diff)
                    .sum()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY8_EXAMPLE, 12), // 23 - 11
                (r#""\\\x20\"""#, 7), // 10 - 3
                (self.input(), 1371),
            ],
            test_cases![
                (DAY8_EXAMPLE, 19), // 42 - 23
                (self.input(), 2117),
            ]
        ]
    }
}

impl Default for Day8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day8 {
    pub fn new() -> Self {
        Self { }
    }
    
    fn decode_size_diff(line: &str) -> usize {
        let repr_size = line.len();
        let mut char_size = repr_size-2;
        let mut iter = line.chars();
        // wrapping quotes
        iter.next();
        iter.next_back();
        
        while let Some(c) = iter.next() {
            if c == '\\' {
                char_size -= 1;
                // consumes next char
                if let Some('x') = iter.next() {
                    char_size -= 2;
                    // \x00 - consume 2 more
                    iter.next();
                    iter.next();
                };
            }
        }
        
        repr_size - char_size
    }
    
    fn encode_size_diff(line: &str) -> usize {
        let char_size = line.len();
        let mut repr_size = char_size+2; // wrapping quotes

        for c in line.chars() {
            if matches!(c, '\\' | '"') {
                repr_size += 1; // escaped
            }
        }

        repr_size - char_size
    }
}