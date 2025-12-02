use num::{Float, Integer};

use crate::*;

aoc_day!(
    day = 2,
    output = usize,
    examples = [
"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124"
    ],
    tests = [
        test_cases![
            ("11-22", [11,22].iter().sum()),
            ("95-115", 99),
            ("998-1012", 1010),
            ("1188511880-1188511890", 1188511885),
            ("222220-222224", 222222),
            ("1698522-1698528", 0),
            ("446443-446449", 446446),
            ("38593856-38593862", 38593859),
            ("565653-565659", 0),
            ("824824821-824824827", 0),
            ("2121212118-2121212124", 0),
            (Self::EXAMPLES[0], 1227775554),
            (Self::INPUT, 22062284697),
        ],
        test_cases![
            ("11-22", [11,22].iter().sum()),
            ("95-115", [99,111].iter().sum()),
            ("998-1012", [999,1010].iter().sum()),
            ("1188511880-1188511890", 1188511885),
            ("222220-222224", 222222),
            ("1698522-1698528", 0),
            ("446443-446449", 446446),
            ("38593856-38593862", 38593859),
            ("565653-565659", 565656),
            ("824824821-824824827", 824824824),
            ("2121212118-2121212124", 2121212121),
            (Self::EXAMPLES[0], 4174379265),
            (Self::INPUT, 46666175279),
        ]
    ],
    solve = |input, part| {
        let pow10_table = (0..=16).map(|i| 10usize.pow(i)).collect_vec();
        let r = Regex::new(r#"(?<a>\d+)-(?<b>\d+)"#).unwrap();
        let ids = input.par_split(',')
            .map(|pair| {
                let c = r.captures(pair).unwrap();
                (c.usize("a"), c.usize("b"))
            });
        // tried fancy-regex for backtracking
        // /^(\d+)\1$/ and /^(\d+)\1+$/ are not faster (thank god)
        match part {
            Part::One => {
                ids.map(|(start, end)| {
                    // small trick - all numbers between start/end MAY have the same digit count
                    let digits_start = digits(start);
                    let digits_end = digits(end);
                    let digits_maybe = (digits_start == digits_end).then_some(digits_start);
                    (start..=end).into_par_iter()
                        .map(|i| {
                            let i_digits = digits_maybe.unwrap_or_else(|| digits(i));
                            if i_digits % 2 == 0 && check(i, pow10_table[i_digits/2]) {
                                return i;
                            }
                            0
                        }).sum::<usize>()
                }).sum()
            },
            Part::Two => {
                ids.map(|(start, end)| {
                    let digits_start = digits(start);
                    let digits_end = digits(end);
                    let digits_maybe = (digits_start == digits_end).then_some(digits_start);
                    (start..=end).into_par_iter()
                        .map(|i| {
                            let i_digits = digits_maybe.unwrap_or_else(|| digits(i));
                            for div in (2..=i_digits).filter(|d| i_digits % d == 0) {
                                let cut = pow10_table[i_digits / div];
                                if check(i, cut) {
                                    return i;
                                }
                            }
                            0
                        }).sum::<usize>()
                }).sum()
            }
        }
    }
);

fn digits(num: usize) -> usize {
    (num as f64).log10().ceil() as usize
}

#[inline]
fn check(i: usize, cut: usize) -> bool {
    // i = 1234567890
    // digits = 10; digits/2 = 5 --> cut = 10^5 = 100000
    // 12345..... / 1_00000
    // .....67890 % 1_00000
    let mut curr = i;
    let mut comp = curr % cut;
    while curr > 0 {
        let (next, comp2) = curr.div_mod_floor(&cut);
        if comp != comp2 {
            return false;
        }
        (curr, comp) = (next, comp2);
    }
    true
}