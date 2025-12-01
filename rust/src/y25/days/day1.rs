use crate::*;

aoc_day!(
    day = 1,
    output = usize,
    examples = [
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 3),
            (Self::INPUT, 962),
        ],
        test_cases![
            (Self::EXAMPLES[0], 6),
            (Self::INPUT, 5782),
        ]
    ],
    solve = |input, part| {
        let r = Regex::new(r#"(?<d>[LR])(?<m>\d+)"#).unwrap();
        let instructions = r.captures_iter(input)
            .map(|c| {
                let mut amt = c.isize("m");
                if c.str("d") == "L" {
                    amt = -amt;
                }
                amt
            });
        match part {
            Part::One => {
                instructions.fold((50, 0), |(pos, clicks), amt| {
                    let mut new_pos = pos + amt;
                    if new_pos < 0 {
                        new_pos += 100;
                    }
                    new_pos = new_pos % 100;
                    // println!("from {pos} {el} to {new_pos}");
                    if new_pos == 0 {
                        (0, clicks+1)
                    } else {
                        (new_pos, clicks)
                    }
                }).1
            },
            Part::Two => {
                instructions.fold((50, 0), |(pos, clicks), amt| {
                    let mut new_pos = pos;
                    let mut new_clicks = clicks;

                    new_clicks += amt.abs() as usize / 100; // there are some outliers >100 (like L900)
                    new_pos += amt % 100;
                    while new_pos < 0 {
                        new_clicks += 1;
                        new_pos += 100;
                    }
                    while new_pos >= 100 {
                        new_clicks += 1;
                        new_pos -= 100;
                    }
                    // 5 -> 0 (click!)
                    if new_pos == 0 && amt < 0 {
                        new_clicks += 1;
                    }
                    // 0 -> -5 (95) (no click!)
                    if pos == 0 && amt < 0 {
                        new_clicks -= 1;
                    }
                    if !cfg!(test) {
                        println!("from {pos} {amt} to {} ({new_pos}) {} clicks", pos+amt, new_clicks - clicks);
                    }
                    (new_pos, new_clicks)
                }).1
            }
        }
    }
);
