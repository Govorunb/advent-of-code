use crate::*;

aoc_day!(
    day = 8,
    output = usize,
    examples = [
r#"""
"abc"
"aaa\"aaa"
"\x27"
"#
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 12), // 23 - 11
            (r#""\\\x20\"""#, 7), // 10 - 3
            (Self::INPUT, 1371),
        ],
        test_cases![
            (Self::EXAMPLES[0], 19), // 42 - 23
            (Self::INPUT, 2117),
        ]
    ],
    solve = |input, part| {
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
);

impl Day8 {
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
