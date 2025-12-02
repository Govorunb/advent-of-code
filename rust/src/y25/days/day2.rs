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
            ("30-400,30-4000,30-40000,3-4000000", 496111476),
            // ("1-4294967296", 87729849870725),
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
            // ("1-4294967296", 88304989965662), // uh oh
        ]
    ],
    solve = |input, part| {
        let pow10_table = (0..=12).map(|i| 10usize.pow(i)).collect_vec();
        let factors_table = (0..=12).map(|i| primes::factors_uniq(i)).collect_vec(); // luck? all the digits had only prime factors
        let find_invalid = |(start, end, parts): (_, _, usize)| {
            let digits_start = digits(start);
            let digits_end = digits(end);
            (digits_start..=digits_end)
                .filter(|d| d % parts == 0)
                .flat_map(|digit| {
                    // example:
                    // i = 12345678; digits = 8; reps = 2 (like part 1)
                    // part_size = 8/2 = 4
                    //  --> cut = 10^4 = 10000
                    // 1234.... / 1_0000
                    // ....5678 % 1_0000
                    let part_size = digit/parts;
                    let cut: usize = pow10_table[part_size];
                    let [digit_min, digit_max] = [digit-1, digit].map(|d| pow10_table[d]);

                    // since the number is just (top part)(top part again), we only really need to iterate over the top parts
                    // i think that's like O(sqrt(N)) or something
    
                    // there can be a max of 1 invalid ID for each distinct top part (for each A there is only one AA/AAA/...)
                    // also something interesting: e.g. 12341234 is always divisible by 10001
                    let start_top = start.max(digit_min) / cut.pow(parts as u32 - 1); // e.g.: reps = 4; part_size = 2; cut = 100
                    let end_top = end.min(digit_max-1) / cut.pow(parts as u32 - 1);   // 12_xx_xx_xx / (100^3) = 12
                    // println!("{start}..={end} -> {digit_min}..{digit_max}: {start_top}..={end_top} {reps}");
                    (start_top..=end_top)
                        // 12340000 + 1234; also known as 1234 * 10001 ;)
                        // 12121212 is 12 * 01010101... TODO
                        .map(move |top| (0..parts).fold(0, move |acc, _| acc * cut + top))
                        .skip_while(|&id| id < start)
                        .take_while(|&id| id <= end)
                }).collect_vec()
        };
        
        let ranges = input.trim().split(',')
            .map(|pair| {
                let (start, end) = pair.split_once('-').unwrap();
                let [start, end] = [start, end].map(|s| s.parse().expect(s));
                (start, end)
            });
        match part {
            Part::One => {
                ranges.flat_map(|(start, end)| find_invalid((start, end, 2)))
                    .sum::<usize>()
            },
            Part::Two => {
                ranges.map(|(start, end)| {
                    let digits_start = digits(start);
                    let digits_end = digits(end);

                    let results2 = (digits_start..=digits_end)
                        .map(|d| factors_table[d].iter()
                            .map(|&rep| find_invalid((start, end, rep as usize)))
                            .flatten()
                            .unique()
                        )
                        .flatten();

                    results2.sum::<usize>()
                }).sum::<usize>()
            }
        }
    }
);

fn digits(num: usize) -> usize {
    (num.ilog10() + 1) as usize
}
