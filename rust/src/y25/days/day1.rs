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
        let mut pos = 50;
        let mut clicks = 0;
        match part {
            Part::One => {
                for amt in instructions {
                    let new_pos = (pos + amt) % 100; // pos is (-100..100)
                    // println!("from {pos} {amt} to {new_pos}");
                    if new_pos == 0 {
                        clicks += 1;
                    }
                    pos = new_pos;
                }
            },
            Part::Two => {
                for amt in instructions {
                    clicks += amt.abs() as usize / 100; // there are some outliers >100 (like L900)
                    let new_pos = (pos + amt).rem_euclid(100); // [0..100)
                    if pos != 0 && new_pos != 0 {
                        if amt > 0 && new_pos <= pos { // wrapped over
                            clicks += 1;
                        } else if amt < 0 && new_pos >= pos { // wrapped under
                            clicks += 1;
                        }
                    }
                    if new_pos == 0 {
                        clicks += 1;
                    }
                    // println!("from {pos} {amt} to {} ({new_pos})", pos+amt);
                    pos = new_pos;
                }
            }
        }
        clicks
    }
);
