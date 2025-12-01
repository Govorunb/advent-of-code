use crate::*;

aoc_day!(
    day = 6,
    output = String,
    examples = [
"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
",
],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], "easter".into()),
            (Self::INPUT, "liwvqppc".into()),
        ],
        test_cases![
            (Self::EXAMPLES[0], "advent".into()),
            (Self::INPUT, "caqfbzlh".into()),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        let cols = lines.clone().nth(0).unwrap().len();
        let mut counts = vec![vec![0u32; 26]; cols];
        for l in lines {
            for (i, c) in l.chars().enumerate() {
                let col_counts = &mut counts[i];
                col_counts[((c as u8) - b'a') as usize] += 1;
            }
        }
        match part {
            Part::One => {
                counts.into_iter().map(|cts| (cts.iter().position_max().unwrap() as u8 + b'a') as char).collect()
            },
            Part::Two => {
                counts.into_iter().map(|cts| (cts.iter().map(|&c| if c == 0 {u32::MAX} else {c}).position_min().unwrap() as u8 + b'a') as char).collect()
            }
        }
    }
);
