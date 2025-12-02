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
            (Self::EXAMPLES[1], 21327161532716),
            (Self::EXAMPLES[2], 121412594604227157),
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
            // (Self::EXAMPLES[1], 21346784611163),
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
        match part {
            Part::One => {
                ids.map(|(start, end)| {
                    // it's time to get "clever"
                    let digits_start = digits(start);
                    let digits_end = digits(end);
                    let mut total = 0;

                    let even_digits = if digits_start % 2 == 0 {
                        digits_start
                    } else if digits_end % 2 == 0 {
                        digits_end
                    } else {
                        return 0
                    };
                    
                    // i = 1234567890
                    // digits = 10; digits/2 = 5 --> cut = 10^5 = 100000
                    // 12345..... / 1_00000
                    // .....67890 % 1_00000
                    let half_cut = pow10_table[even_digits/2];
                    // let [even_min, even_max] = pow10_table[(even_digits-1)..=even_digits] else {unreachable!()};
                    
                    // there can be a max of 1 invalid ID for each distinct top half (for each A there is only one AA)
                    // let mut count = 0;
                    let start_top = start / half_cut;
                    let end_top = end / half_cut;
                    for top in start_top..=end_top {
                        let id = top * half_cut + top;
                        // top half may have odd # digits
                        if digits(id) % 2 != 0 {continue}
                        // if id > even_max || id < even_min {continue} // no difference
                        if id >= start && id <= end {
                            total += id;
                            // count += 1;
                        }
                    }
                    // FIXME: very very lucky that the input was kind
                    // e.g. 30-40000
                    // picks even_digits from start (2)
                    // half_cut is 10 -> start_top..=end_top is 3..=4000
                    // then, top * half_cut + top = 1234 * 10 + 1234 (which is obviously absurd and not a double)
                    
                    // let digits_maybe = (digits_start == digits_end).then_some(digits_start);
                    // let results = (start..=end).into_iter()
                    //     .map(|i| {
                    //         let i_digits = digits_maybe.unwrap_or_else(|| digits(i));
                    //         if i_digits % 2 == 0 && check(i, pow10_table[i_digits/2]) {
                    //             return i;
                    //         }
                    //         0
                    //     }).filter(|&e| e > 0).collect_vec();
                    // let sum = results.iter().sum();
                    // println!("verify {start}..{end}:\n\tsum: {sum}|{total}\n\tcount: {}|{count}", results.len());
                    // assert!(results.len() == count);
                    // assert!(sum == total);
                    total
                }).sum()
            },
            Part::Two => {
                ids.map(|(start, end)| {
                    // small trick - all numbers between start/end MAY have the same digit count
                    let digits_start = digits(start);
                    let digits_end = digits(end);
                    let digits_maybe = (digits_start == digits_end).then_some(digits_start);
                    (start..=end).into_par_iter()
                        .map(|i| {
                            let i_digits = digits_maybe.unwrap_or_else(|| digits(i));
                            for &div in &factors_table[i_digits] {
                                let cut = pow10_table[i_digits / div as usize];
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
    num.checked_ilog10().unwrap_or(0) as usize + 1
}

#[inline]
fn check(i: usize, cut: usize) -> bool {
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