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
            (Self::EXAMPLES[1], 21327161532716), // actually lower due to overlapping ranges
            (Self::EXAMPLES[2], 121412594604227157),
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
        let factors_table = (0..15).map(|i| primes::factors(i)).collect_vec();
        let r = Regex::new(r#"(?<a>\d+)-(?<b>\d+)"#).unwrap();
        let ids = input.par_split(',')
            .map(|pair| {
                // parsing later might save a negligible amount of time on log10
                let c = r.captures(pair).unwrap();
                (c.usize("a"), c.usize("b"))
            });
        // tried fancy-regex for backtracking
        // /^(\d+)\1$/ and /^(\d+)\1+$/ are way slower (thank god)

        let buh = |(start, end, divisor): (_, _, usize)| {
            // it's time to get "clever"
            let digits_start = digits(start);
            let digits_end = digits(end);
            let mut total = 0;
            for digit in (digits_start..=digits_end).filter(|d| d % divisor == 0) {
                // i = 12345678
                // digits = 8; digits/2 = 4 --> cut = 10^4 = 10000
                // 1234.... / 1_0000
                // ....5678 % 1_0000
                let half_cut: usize = pow10_table[digit/divisor];
                let [even_min, even_max] = pow10_table[(digit-1)..=digit] else {unreachable!()};
                
                // there can be a max of 1 invalid ID for each distinct top half (for each A there is only one AA/AAA/...)
                // also something interesting: e.g. 12341234 is always divisible by 10001
                let start_top = start.max(even_min) / half_cut;
                let end_top = end.min(even_max-1) / half_cut;
                let invalid_ids = (start_top..=end_top)
                    .map(|top| top * half_cut + top) // 12340000 + 1234; also known as 10001 * 1234 ;)
                // let start_n = start.next_multiple_of(repeat_multiple) / repeat_multiple;
                // let end_n = end.prev_multiple_of(&(repeat_multiple)) / repeat_multiple;
                // let invalid_ids = (start_n..=end_n)
                //     .map(|n| n * repeat_multiple) // 12340000 + 1234; also known as 10001 * 1234 ;)
                //     // this increments by 1, making the multiplied product increment by 10001
                    .skip_while(|&id| id < start)
                    .take_while(|&id| id <= end);
                total += invalid_ids.sum::<usize>();
            }
            total
        };

        match part {
            Part::One => {
                ids.map(|(start, end)| {
                    buh((start, end, 2))
                }).sum()
            },
            Part::Two => {
                ids.map(|(start, end)| {
                    let digits_start = digits(start);
                    let digits_end = digits(end);
                    
                    // let mut total = 0;
                    // for digit in digits_start..=digits_end  {
                    //     for &divisor in &factors_table[digit] {
                    //         let part_size = digit / divisor as usize;
                    //         let cut = pow10_table[part_size];
                    //         let [digit_min, digit_max] = pow10_table[(digit-1)..=digit] else {unreachable!()};

                    //         let start_top = start.max(digit_min) / cut;
                    //         let end_top = end.min(digit_max) / cut;
                    //     }
                    // }
                    
                    let digits_maybe = (digits_start == digits_end).then_some(digits_start);
                    (start..=end).into_par_iter()
                        .map(|i| {
                            let i_digits = digits_maybe.unwrap_or_else(|| digits(i));
                            for &rep in factors_table[i_digits].iter() {
                                let rep = rep as usize;
                                let p_size = i_digits / rep;
                                let cut = pow10_table[p_size];
                                let is_invalid = {
                                    // 1001 -> 1001001001001 (rep 3)
                                    // 123123123 = 123 * 001001001
                                    if i_digits % rep != 0 {
                                        false
                                    } else {
                                        let mut repeat_multi = 1;
                                        for _ in 0..(rep-1) {
                                            repeat_multi *= cut;
                                            repeat_multi += 1;
                                        }
    
                                        let (div, rem) = i.div_rem(&repeat_multi);
                                        div < cut && rem == 0
                                    }
                                };
                                // let is_invalid = check_orig(i, cut);
                                // assert!(is_invalid == check_orig(i, cut));
                                if is_invalid {
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
    num.checked_ilog10().unwrap_or(0) as usize + 1
}

fn check_orig(i: usize, cut: usize) -> bool {
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
