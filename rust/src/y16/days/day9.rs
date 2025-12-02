use std::ascii::Char;

use crate::*;

aoc_day!(
    day = 9,
    output = usize,
    examples = [],
    tests = [
        test_cases![
            ("ADVENT", 6),
            ("A(1x5)BC", 7),
            ("(3x3)XYZ", 9),
            ("A(2x2)BCD(2x2)EFG", 11),
            ("(6x1)(1x3)A", 6),
            ("X(8x2)(3x3)ABCY", 18),
            (Self::INPUT, 97714),
        ],
        test_cases![
            ("(3x3)XYZ", 9),
            ("X(8x2)(3x3)ABCY", 20),
            ("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920),
            ("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEJ", 445),
            (Self::INPUT, 10762972461),
        ]
    ],
    solve = |input, part| {
        parser(input.as_bytes(), part)
    }
);

fn parser(s: &[u8], part: Part) -> usize {
    let mut total = 0;
    let mut i = 0;
    while i < s.len() {
        let b = s[i];
        i += 1;
        if let b'(' = b {
            let start = i;
            let decomp_block_len = s[i..].iter().position(|&c| c == b')').unwrap();
            // dbg!((s[i..].as_str(), decomp_block_len, start, i, total));
            i += decomp_block_len;
            let decomp_instr = &s[start..i];
            i += 1; // b')'
            // dbg!(decomp_instr.as_str());
            let (read, repeat) = decomp_instr.split_once(|&c| c == b'x').unwrap();
            let [read, repeat] = [read, repeat].map(|r| r.as_str().parse::<usize>().unwrap());
            match part {
                Part::One => {
                    i += read;
                    total += repeat * read;
                },
                Part::Two => {
                    let repeated = &s[i..(i+read)];
                    i += read;
                    total += repeat * parser(repeated, part);
                }
            }
        } else {
            total += 1;
        }
        assert!(i < (s.len() + 1));
    }
    total
}
