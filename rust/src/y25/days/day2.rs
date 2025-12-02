use std::iter;

use num::{Float, Integer};

use crate::*;

aoc_day!(
    day = 2,
    output = usize,
    examples = [
"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124",
// additional challenges
"11-42,95-115,998-7012,1188511880-2188511890,222220-222224,1698522-1698528,446443-646449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2321212124",
"11-42,95-115,998-7012,222220-222224,446443-646449,1698522-1698528,38593856-38593862,824824821-824824827,1188511880-2321212124,202001202277-532532532530",
// death
"98765432-1234567890,1000000000000000000000000-1500000000000000000000000,988970940900875998011400-1050032916531789321707634,123456789012345678901234567890-234567890123456789012345678901",
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
            // (Self::EXAMPLES[1], 21327161532716), // actually lower due to overlapping ranges
            // (Self::EXAMPLES[2], 121412594604227157),
            ("30-400,30-4000,30-40000, 3-4000000", 496111476),
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
            // too long
            // (Self::EXAMPLES[1], 21346784611163), // lower (overlap)
            // (Self::EXAMPLES[2], 0),
        ]
    ],
    solve = |input, part| {
        let pow10_table = (0..=15).map(|i| 10usize.pow(i)).collect_vec();
        let factors_table = (0..15).map(|i| primes::factors_uniq(i)).collect_vec();
        let r = Regex::new(r#"(?<a>\d+)-(?<b>\d+)"#).unwrap();
        // tried fancy-regex for backtracking
        // /^(\d+)\1$/ and /^(\d+)\1+$/ are way slower (thank god)

        let buh = |(start, end, reps): (_, _, usize)| {
            // it's time to get "clever"
            let digits_start = digits(start);
            let digits_end = digits(end);
            (digits_start..=digits_end)
                .filter(|d| d % reps == 0)
                .flat_map(|digit| {

                    // i = 12345678
                    // digits = 8; digits/2 = 4 --> cut = 10^4 = 10000
                    // 1234.... / 1_0000
                    // ....5678 % 1_0000
                    let part_size = digit/reps;
                    let cut: usize = pow10_table[part_size];
                    let [digit_min, digit_max] = pow10_table[(digit-1)..=digit] else {unreachable!()};
    
                    // there can be a max of 1 invalid ID for each distinct top part (for each A there is only one AA/AAA/...)
                    // also something interesting: e.g. 12341234 is always divisible by 10001
                    let start_top = start.max(digit_min) / cut.pow(reps as u32 - 1);
                    let end_top = end.min(digit_max-1) / cut.pow(reps as u32 - 1);
                    // println!("{start}..={end} -> {digit_min}..{digit_max}: {start_top}..={end_top} {reps}");
                    (start_top..=end_top)
                        .map(move |top| (0..reps).fold(0, move |acc, _| acc * cut + top)) // 12340000 + 1234; also known as 1234 * 10001 ;)
                        .skip_while(|&id| id < start)
                        .take_while(|&id| id <= end)
                }).collect_vec()
        };
        
        let ids = input.split(',')
            .map(|pair| {
                let c = r.captures(pair).unwrap();
                (c.usize("a"), c.usize("b"))
            });
        match part {
            Part::One => {
                ids.flat_map(|(start, end)| buh((start, end, 2)))
                    .sum::<usize>()
            },
            Part::Two => {
                ids.map(|(start, end)| {
                    let digits_start = digits(start);
                    let digits_end = digits(end);

                    let results2 = (digits_start..=digits_end)
                        .flat_map(|d| factors_table[d].iter()
                            .map(|&rep| buh((start, end, rep as usize)))
                    ).flatten();

                    results2.unique().sum::<usize>()
                }).sum::<usize>()
            }
        }
    }
);

fn digits(num: usize) -> usize {
    num.checked_ilog10().unwrap_or(0) as usize + 1
}
